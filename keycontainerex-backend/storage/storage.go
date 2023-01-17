package storage

import (
	"os"
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
	FilePath string
	KeyHash  []byte
	Password []*Password
}

func NewStorage(storagePath string) *Storage {
	return &Storage{
		FilePath: storagePath,
	}
}

func (s *Storage) Save() error {
	_, err := os.Stat(s.FilePath)
	if !os.IsNotExist(err) {
		if err = os.Remove(s.FilePath); err != nil {
			return err
		}
	}
	var data []byte
	data = append(data, []byte{0x4B, 0x43, 0x45, 0x58}...)
	if err = os.WriteFile(s.FilePath, data, 0600); err != nil {
		return err
	}
	return nil
}
