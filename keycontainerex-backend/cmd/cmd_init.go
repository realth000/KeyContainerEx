package cmd

import (
	"KeyContainerEx/common"
	"KeyContainerEx/log"
	"KeyContainerEx/secure"
	"KeyContainerEx/storage"
	"KeyContainerEx/util"
	"fmt"
	"github.com/realth000/ToGoTool/crypto/hash"
	"os"
)

const (
	stdin = common.Stdin
)

func checkInit(storagePath string) (bool, error) {
	if storagePath == "" {
		return false, fmt.Errorf("empty storage path")
	}
	info, err := os.Stat(storagePath)
	if os.IsNotExist(err) {
		return false, nil
	} else if err == nil || os.IsExist(err) {
		if info.IsDir() {
			return false, fmt.Errorf("not a file: %s", storagePath)
		}
		// Validate storage.
		err = storage.CheckStorage(storagePath)
		if err != nil {
			return false, err
		}
		return true, nil
	}
	return false, err
}

func initialize(storagePath string) (*storage.Storage, error) {
	fmt.Println("Initializing storage...")
	pw, err := util.ReadPassword("Input main password: ")
	if err != nil {
		return nil, err
	}
	cpw, err := util.ReadPassword("Confirm main password: ")
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
	return storage.NewStorage(storagePath, mainPassword), nil
}
