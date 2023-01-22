package storage

import (
	"KeyContainerEx/secure"
	"encoding/binary"
	"errors"
	"fmt"
	"github.com/realth000/ToGoTool/crypto/aes"
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
	if err = s.loadMainPassword(f); err != nil {
		return err
	}
	s.passwordKey, err = aes.GenerateAESKeyFromBytes(s.MainPassword.GetHash(), aes.Type256)
	if err != nil {
		return fmt.Errorf("failed to generate AES key from main password: %w", err)
	}
	if err = s.loadPassword(f); err != nil {
		// When loadPassword returns a safeEOF, it means we loaded all password data from the
		// storage file, all entire password, no half ones.
		// Normally finished, not an error.
		if errors.Is(err, safeEOF) {
			return nil
		}
		return err
	}

	// Load password.
	return nil
}

func (s *Storage) loadMainPassword(f *os.File) error {
	var err error
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
	err = binary.Read(f, binary.LittleEndian, &mpBuffer)
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
		return fmt.Errorf("broken storage: missing boundary after main password: %w", err)
	}
	if mpSplit != mainPasswordSplit {
		return fmt.Errorf("broken storage: invalid boundary after main password: %0x", mpSplit)
	}
	//fmt.Printf("main password type=%d, hash=%0x\n", s.MainPassword.HashType, s.MainPassword.GetHash())
	return nil
}

func readPart[T int8 | uint32 | []byte](
	f io.Reader,
	target *T,
	message string,
	isInvalidFunc func() bool,
) error {
	if err := binary.Read(f, binary.LittleEndian, target); err != nil {
		return fmt.Errorf("error reading %s: %w", message, err)
	}
	if isInvalidFunc != nil && isInvalidFunc() {
		return fmt.Errorf("invalid %s: %v", message, target)
	}
	return nil
}

func (s *Storage) loadPassword(f *os.File) error {
	var err error
	for {
		var cryptoType, cryptoMode int8
		var accountLength, passwordLength, commentLength, createdTimeLength, lastModifiedTimeLength uint32
		var accountData, passwordData, commentData, createdTimeData, lastModifiedTimeData []byte
		// TODO: Bad code.
		if err = readPart(f, &cryptoType, "password crypto type",
			func() bool { return cryptoType > int8(CryptoAES256) }); err != nil {
			if errors.Is(err, io.EOF) {
				break
			} else {
				return err
			}
		}
		if err = readPart(f, &cryptoMode, "password crypto length",
			func() bool { return cryptoMode > CryptoModeCTR }); err != nil {
			return err
		}
		if err = readPart(f, &accountLength, "account length", nil); err != nil {
			return err
		}
		accountData = make([]byte, accountLength)
		if err = readPart(f, &accountData, "account data", nil); err != nil {
			return err
		}
		if err = readPart(f, &commentLength, "comment length", nil); err != nil {
			return err
		}
		commentData = make([]byte, commentLength)
		if err = readPart(f, &commentData, "comment data", nil); err != nil {
			return err
		}
		if err = readPart(f, &passwordLength, "password length", nil); err != nil {
			return err
		}
		passwordData = make([]byte, passwordLength)
		if err = readPart(f, &passwordData, "password data", nil); err != nil {
			return err
		}

		if err = readPart(f, &createdTimeLength, "createdTime length", nil); err != nil {
			return err
		}
		createdTimeData = make([]byte, createdTimeLength)
		if err = readPart(f, &createdTimeData, "createdTime data", nil); err != nil {
			return err
		}
		if err = readPart(f, &lastModifiedTimeLength, "lastModifiedTime length", nil); err != nil {
			return err
		}
		lastModifiedTimeData = make([]byte, lastModifiedTimeLength)
		if err = readPart(f, &lastModifiedTimeData, "lastModifiedTime data", nil); err != nil {
			return err
		}
		var split byte
		if err = binary.Read(f, binary.LittleEndian, &split); err != nil {
			return fmt.Errorf("broken storage: missing boundary after password: %w", err)
		}
		if split != passwordSplit {
			return fmt.Errorf("broken storage: invalid boundary after password: %0x", split)
		}
		p, err := secure.NewPasswordFromHash(
			accountData,
			passwordData,
			commentData,
			createdTimeData,
			lastModifiedTimeData,
			*secure.NewPasswordOption(
				aes.Type256,
				aes.ModeCFB,
				s.passwordKey,
			),
		)
		if err != nil {
			return fmt.Errorf("failed to make password: %w", err)
		}
		if err = s.AddPassword(p); err != nil {
			return fmt.Errorf("failed to add password: %w", err)
		}
	}
	// We return a safeEOF error to indicate loading password is finished with EOF and
	// the finish position is "safe": Just loaded the last entire password data, no half
	// password data in loading.
	return safeEOF
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
	err = binary.Read(f, binary.LittleEndian, &storageHeader)
	if err != nil {
		return fmt.Errorf("error reading storage header: %w", err)
	}
	if !reflect.DeepEqual(storageHeader, MagicHeader) {
		return fmt.Errorf("invalid storage header: %0x", storageHeader)
	}
	return nil
}
