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

func NewPasswordOption(aesType aes.Type, aesMode aes.Mode, aesKey []byte) *PasswordOption {
	return &PasswordOption{
		AESType: aesType,
		AESMode: aesMode,
		aesKey:  aesKey,
	}
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

func (p *Password) Account() (string, error) {
	s, err := aes.Decrypt(p.Option.AESMode, p.Option.aesKey, p.account)
	if err != nil {
		return "", err
	}
	return string(s), nil
}

func (p *Password) Password() (string, error) {
	s, err := aes.Decrypt(p.Option.AESMode, p.Option.aesKey, p.password)
	if err != nil {
		return "", err
	}
	return string(s), nil
}

func (p *Password) Comment() (string, error) {
	c, err := aes.Decrypt(p.Option.AESMode, p.Option.aesKey, p.password)
	if err != nil {
		return "", err
	}
	return string(c), nil
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

func (p *Password) Validate() error {
	var err error
	//fmt.Printf("AAAA validate in mode %d, key=%0x\n", p.Option.AESMode, p.Option.aesKey)
	if _, err = aes.Decrypt(p.Option.AESMode, p.Option.aesKey, p.account); err != nil {
		return fmt.Errorf("invalid account: %w", err)
	}
	if _, err = aes.Decrypt(p.Option.AESMode, p.Option.aesKey, p.password); err != nil {
		return fmt.Errorf("invalid password: %w", err)
	}
	if _, err = aes.Decrypt(p.Option.AESMode, p.Option.aesKey, p.comment); err != nil {
		return fmt.Errorf("invalid comment: %w", err)
	}
	c, err := aes.Decrypt(p.Option.AESMode, p.Option.aesKey, p.createdTime)
	if err != nil {
		return fmt.Errorf("invalid createdTime: %w", err)
	}
	_, err = strconv.ParseInt(slice.ByteToString(c), 10, 64)
	if err != nil {
		return fmt.Errorf("failed to parse createdTime: %w", err)
	}
	l, err := aes.Decrypt(p.Option.AESMode, p.Option.aesKey, p.createdTime)
	if err != nil {
		return fmt.Errorf("invalid lastModifiedTime: %w", err)
	}
	_, err = strconv.ParseInt(slice.ByteToString(l), 10, 64)
	if err != nil {
		return fmt.Errorf("failed to parse lastModifiedTime: %w", err)
	}
	return nil
}

func NewPassword(account string, password string, comment string, option PasswordOption) (*Password, error) {
	k, err := aes.GenerateAESKeyFromBytes(option.aesKey, option.AESType)
	if err != nil {
		return nil, fmt.Errorf("failde to generate AES key: %w", err)
	}
	option.aesKey = k
	p := Password{
		Option: option,
	}

	err = p.encrypt(account, password, comment)
	// fmt.Printf("type=%d, mode=%d, key=%0x\n", option.AESType, option.AESMode, option.aesKey)
	// fmt.Printf("%0x\n", p.account)
	// fmt.Printf("%0x\n", p.password)
	// fmt.Printf("%0x\n", p.comment)
	// fmt.Printf("%0x\n", p.createdTime)
	// fmt.Printf("%0x\n", p.lastModifiedTime)
	if err != nil {
		return nil, err
	}
	return &p, nil
}

func NewPasswordFromHash(
	accountHash []byte,
	passwordHash []byte,
	commentHash []byte,
	createdTimeHash []byte,
	lastModifiedTimeHash []byte,
	option PasswordOption,
) (*Password, error) {
	p := Password{
		Option:           option,
		account:          accountHash,
		password:         passwordHash,
		comment:          commentHash,
		createdTime:      createdTimeHash,
		lastModifiedTime: lastModifiedTimeHash,
	}
	// fmt.Printf("type=%d, mode=%d, key=%0x\n", option.AESType, option.AESMode, option.aesKey)
	// fmt.Printf("%0x\n", accountHash)
	// fmt.Printf("%0x\n", passwordHash)
	// fmt.Printf("%0x\n", commentHash)
	// fmt.Printf("%0x\n", createdTimeHash)
	// fmt.Printf("%0x\n", lastModifiedTimeHash)
	if err := p.Validate(); err != nil {
		return nil, err
	}
	return &p, nil
}
