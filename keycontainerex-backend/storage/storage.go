package storage

import (
	"KeyContainerEx/secure"
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
	Password     []*secure.Password
}

func NewStorage(storagePath string, password *secure.MainPassword) *Storage {
	return &Storage{
		FilePath:     storagePath,
		MainPassword: password,
	}
}

func NewEmptyStorage(storagePath string) *Storage {
	return &Storage{
		FilePath: storagePath,
	}
}
