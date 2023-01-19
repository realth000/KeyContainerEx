package storage

import (
	"fmt"
	"io"
	"os"
	"reflect"
)

func (s *Storage) Load() error {
	_, err := os.Stat(s.FilePath)
	if err != nil && !os.IsExist(err) {
		return err
	}
	var f *os.File
	f, err = os.Open(s.FilePath)
	if err != nil {
		return err
	}
	defer f.Close()
	storageHeader := make([]byte, len(MagicHeader))
	_, err = io.ReadFull(f, storageHeader)
	if err != nil {
		return fmt.Errorf("error reading storage header: %w", err)
	}
	if !reflect.DeepEqual(storageHeader, MagicHeader) {
		return fmt.Errorf("invalid storage header: %0x", storageHeader)
	}
	fmt.Printf("storage header: %0x\n", storageHeader)
	return nil
}
