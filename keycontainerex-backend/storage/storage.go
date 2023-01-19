package storage

import (
	"KeyContainerEx/secure"
)

var (
	MagicHeader = []byte{0x4B, 0x43, 0x45, 0x58}
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
