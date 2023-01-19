package storage

import (
	"KeyContainerEx/secure"
	"encoding/binary"
	"fmt"
	"github.com/realth000/ToGoTool/crypto/hash"
	"io"
	"os"
	"reflect"
)

func (s *Storage) Load() error {
	if err := CheckStorage(s.FilePath); err != nil {
		return fmt.Errorf("failed to load storage, check not pass: %w", err)
	}
	return s.loadStorage()
}

// loadStorage load Storage from disk file.
// It does NOT validate storage file, so call CheckStorage before it.
func (s *Storage) loadStorage() error {
	var f *os.File
	var err error
	f, err = os.Open(s.FilePath)
	if err != nil {
		return err
	}
	defer f.Close()
	header := make([]byte, len(MagicHeader))
	// Skip header: read and not check.
	_, err = io.ReadFull(f, header)
	if err != nil {
		return fmt.Errorf("error loading storage header: %w", err)
	}
	// Read version:
	// Now version means nothing, just skip.
	var version byte
	err = binary.Read(f, binary.LittleEndian, &version)
	if err != nil {
		return fmt.Errorf("error loading storage version: %w", err)
	}
	var mpType int8
	err = binary.Read(f, binary.LittleEndian, &mpType)
	if err != nil {
		return fmt.Errorf("invalid main password hash type: %w", err)
	}
	if mpType <= -1 || mpType >= int8(HashMax) {
		return fmt.Errorf("invalid main password hash type: %d", mpType)
	}
	var mpSize int32
	err = binary.Read(f, binary.LittleEndian, &mpSize)
	if err != nil {
		return fmt.Errorf("error loading main password hash length: %s", err)
	}
	mpBuffer := make([]byte, mpSize)
	_, err = io.ReadFull(f, mpBuffer)
	if err != nil {
		return fmt.Errorf("error loading main password hash: %w %d", err, mpSize)
	}
	if len(mpBuffer) < 1 {
		return fmt.Errorf("invalid empty main password readed from storage")
	}
	s.MainPassword = secure.NewMainPasswordWithHash(mpBuffer, hash.SumType(mpType))
	var mpSplit byte
	err = binary.Read(f, binary.LittleEndian, &mpSplit)
	if err != nil {
		return fmt.Errorf("broken storage: missing boundary after main password")
	}
	if mpSplit != mainPasswordSplit {
		return fmt.Errorf("broken storage: invalid boundary after main password: %0x", mpSplit)
	}
	fmt.Printf("main password type=%d, hash=%0x\n", s.MainPassword.HashType, s.MainPassword.GetHash())

	// Load password.
	return nil
}

func CheckStorage(filePath string) error {
	var f *os.File
	var err error
	_, err = os.Stat(filePath)
	if err != nil && !os.IsExist(err) {
		return err
	}
	f, err = os.Open(filePath)
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
	return nil
}
