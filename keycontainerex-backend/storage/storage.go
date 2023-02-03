package storage

import (
	"KeyContainerEx/secure"
	"errors"
	"fmt"
	"github.com/realth000/ToGoTool/crypto/aes"
	"github.com/realth000/ToGoTool/crypto/hash"
)

type HashType = hash.SumType

const (
	HashMD5        = hash.SumMD5
	HashSHA1       = hash.SumSHA1
	HashSHA224     = hash.SumSHA224
	HashSHA256     = hash.SumSHA256
	HashSHA384     = hash.SumSHA384
	HashSHA512     = hash.SumSHA512
	HashSHA512_256 = hash.SumSHA512_256
	HashSHA3_224   = hash.SumSHA3_224
	HashSHA3_256   = hash.SumSHA3_256
	HashSHA3_384   = hash.SumSHA3_384
	HashSHA3_512   = hash.SumSHA3_512
	HashMax        = HashSHA3_512 + 1
)

type CryptoType = aes.Type

const (
	CryptoInvalid = aes.TypeInvalid
	CryptoAES128  = aes.Type128
	CryptoAES192  = aes.Type192
	CryptoAES256  = aes.Type256
)

type CryptoMode = aes.Mode

const (
	CryptoModeCBC = aes.ModeCBC
	CryptoModeCFB = aes.ModeCFB
	CryptoModeOFB = aes.ModeOFB
	CryptoModeCTR = aes.ModeCTR
)

var (
	MagicHeader       = []byte{0x4B, 0x43, 0x45, 0x58}
	version           = byte(0x01)
	mainPasswordSplit = byte(0xa1)
	passwordSplit     = byte(0xa2)
	safeEOF           = errors.New("EOF(safely finished)")
)

/*
	Storage format:
	Bytes	Section
	4		File Header (0x4B434558, KCEX)
	32		Key SHA-512/256 hash
	32		Password data section SHA-512/256 hash
	-		Password data
*/

type Storage struct {
	FilePath     string
	MainPassword *secure.MainPassword
	Password     map[string]*secure.Password
	passwordKey  []byte
}

func (s *Storage) AddPassword(password *secure.Password) error {
	if password == nil {
		return fmt.Errorf("nil password")
	}
	id, err := password.Id()
	if err != nil {
		return fmt.Errorf("failed to get password account: %w", err)
	}
	if _, ok := s.Password[id]; ok {
		return fmt.Errorf("account already exists")
	}
	s.Password[id] = password
	return nil
}

func NewStorage(storagePath string, password *secure.MainPassword) (*Storage, error) {
	k, err := aes.GenerateAESKeyFromBytes(password.GetHash(), aes.Type256)
	if err != nil {
		return nil, fmt.Errorf("failed to generate AES key from main password: %w", err)
	}
	return &Storage{
		FilePath:     storagePath,
		MainPassword: password,
		Password:     make(map[string]*secure.Password),
		passwordKey:  k,
	}, nil
}

func NewEmptyStorage(storagePath string) *Storage {
	return &Storage{
		FilePath: storagePath,
		Password: make(map[string]*secure.Password),
	}
}
