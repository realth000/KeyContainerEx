package secure

import (
	"KeyContainerEx/common"
	"fmt"
	"github.com/realth000/ToGoTool/crypto/hash"
	"golang.org/x/crypto/ssh/terminal"
	"reflect"
)

const (
	stdin = common.Stdin
)

type MainPassword struct {
	HashType hash.SumType
	hash     []byte
}

func (m *MainPassword) UpdatePassword(password string) {
	m.hash = hash.HashString(m.HashType, password)
}

func (m *MainPassword) GetHash() []byte {
	return m.hash
}

func (m *MainPassword) RequireMainPassword() error {
	fmt.Print("Input main password: ")
	pw, err := terminal.ReadPassword(stdin)
	if err != nil {
		return err
	}
	fmt.Println("")
	if !reflect.DeepEqual(hash.Hash(m.HashType, pw), m.hash) {
		return fmt.Errorf("main password not correct")
	}
	return nil
}

func NewMainPassword(password string, hashType hash.SumType) *MainPassword {
	m := MainPassword{
		HashType: hashType,
	}
	m.UpdatePassword(password)
	return &m
}

func NewMainPasswordWithHash(hash []byte, hashType hash.SumType) *MainPassword {
	m := MainPassword{
		HashType: hashType,
		hash:     hash,
	}
	return &m
}
