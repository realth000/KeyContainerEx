package cmd

import (
	"KeyContainerEx/log"
	"KeyContainerEx/secure"
	"KeyContainerEx/storage"
	"fmt"
	"github.com/realth000/ToGoTool/crypto/hash"
	"golang.org/x/crypto/ssh/terminal"
	"os"
)

const (
	stdin = 0
)

func checkInit(storagePath string) (bool, error) {
	if storagePath == "" {
		return false, fmt.Errorf("empty storage path")
	}
	info, err := os.Stat(storagePath)
	if err == nil || os.IsExist(err) {
		if info.IsDir() {
			return false, fmt.Errorf("not a file: %s", storagePath)
		}
		// Validate storage.
		return true, nil
	} else if os.IsNotExist(err) {
		return false, nil
	}
	return false, err
}

func initialize(storagePath string) (*storage.Storage, error) {
	fmt.Println("Initializing storage...")
	fmt.Println("Input storage password:")
	pw, err := terminal.ReadPassword(stdin)
	if err != nil {
		return nil, err
	}
	fmt.Println("Confirm storage password:")
	cpw, err := terminal.ReadPassword(stdin)
	if err != nil {
		return nil, err
	}
	spw := string(pw)
	scpw := string(cpw)
	if spw != scpw {
		log.FatalPrintln("password and confirm password not the same, exit")
	}

	mainPassword := secure.NewMainPassword(spw, hash.SumSHA3_512)
	fmt.Printf("AAAAA get main password hash:%0x\n", mainPassword.GetHash())
	return storage.NewStorage(storagePath), nil
}
