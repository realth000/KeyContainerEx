package secure

import (
	"fmt"
	"github.com/realth000/ToGoTool/crypto/aes"
	"github.com/realth000/ToGoTool/slice"
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
	AESType aes.Type
	AESMode aes.Mode
	aesKey  []byte
}

// Password stores all password related encrypted data.
type Password struct {
	Option           PasswordOption
	account          []byte
	password         []byte
	comment          []byte
	createdTime      []byte
	lastModifiedTime []byte
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
	s, err := aes.Decrypt(p.Option.AESMode, p.Option.aesKey, p.password)
	if err != nil {
		return "", err
	}
	return string(s), nil
}

func (p *Password) encrypt(account string, password string, comment string) error {
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
		var err error
		*v.cipherPointer, err = aes.Encrypt(p.Option.AESMode, p.Option.aesKey, slice.ByteFromString(v.plainText))
		if err != nil {
			return fmt.Errorf("failed to encrypt for account %s: %w", account, err)
		}
	}
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
