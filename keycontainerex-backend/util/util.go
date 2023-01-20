package util

import (
	"KeyContainerEx/common"
	"fmt"
	"golang.org/x/crypto/ssh/terminal"
)

func ReadPassword(hint string) ([]byte, error) {
	fmt.Print(hint)
	pw, err := terminal.ReadPassword(common.Stdin)
	if err != nil {
		return nil, err
	}
	fmt.Println("")
	return pw, nil
}
