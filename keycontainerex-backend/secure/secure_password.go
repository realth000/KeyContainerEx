package secure

import (
	"fmt"
	aes "github.com/realth000/ToGoTool/crypto/aes"
	hash "github.com/realth000/ToGoTool/crypto/hash"
	slice "github.com/realth000/ToGoTool/slice"
	"strconv"
	"time"
)

type PasswordStorage struct {
	Account           []byte
	EncryptedPassword []byte
	Comment           []byte
	CreatedTime       []byte
	LastModifiedTime  []byte
}

type PasswordOption struct {
	HashType hash.SumType
	AESKey   []byte
	AESType  aes.Type
	AESMode  aes.Mode
}

// Password stores all password related encrypted data.
type Password struct {
	Option           PasswordOption
	account          []byte
	password         []byte
	comment          []byte
	createdTime      []byte
	lastModifiedTime []byte
	hash             []byte
}

func (p *Password) StorageData() PasswordStorage {
	return PasswordStorage{
		Account:           p.account,
		EncryptedPassword: p.password,
		Comment:           p.comment,
		CreatedTime:       p.createdTime,
		LastModifiedTime:  p.lastModifiedTime,
	}
}

func (p *Password) Password() (string, error) {
	s, err := aes.Decrypt(p.Option.AESMode, p.Option.AESKey, p.password)
	if err != nil {
		return "", err
	}
	return string(s), nil
}

func (p *Password) encrypt(account string, password string, comment string) error {
	var err error
	for _, v := range []struct {
		plainText     string
		cipherPointer *[]byte
	}{
		{
			plainText:     account,
			cipherPointer: &p.account,
		},
		{
			plainText:     password,
			cipherPointer: &p.password,
		},
		{
			plainText:     comment,
			cipherPointer: &p.comment,
		},
		{
			plainText:     strconv.FormatInt(time.Now().Unix(), 10),
			cipherPointer: &p.createdTime,
		},
		{
			plainText:     strconv.FormatInt(time.Now().Unix(), 10),
			cipherPointer: &p.lastModifiedTime,
		},
	} {
		*v.cipherPointer, err = aes.Encrypt(p.Option.AESMode, p.Option.AESKey, slice.ByteFromString(v.plainText))
		if err != nil {
			return fmt.Errorf("failed to encrypt for account %s: %w", account, err)
		}
	}
	err = p.hashSum()
	return err
}

func (p *Password) hashSum() error {
	s, err := p.Password()
	if err != nil {
		return err
	}
	p.hash = hash.HashString(p.Option.HashType, s)
	return nil
}

func NewPassword(account string, password string, comment string, option PasswordOption) (*Password, error) {
	p := Password{
		Option: option,
	}
	err := p.encrypt(account, password, comment)
	if err != nil {
		return nil, err
	}
	return &p, nil
}
