package secure

import (
	"github.com/realth000/ToGoTool/crypto/hash"
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
