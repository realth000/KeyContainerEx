package util

import (
	"KeyContainerEx/common"
	"bufio"
	"fmt"
	"github.com/realth000/ToGoTool/slice"
	"golang.org/x/crypto/ssh/terminal"
	"os"
	"strings"
)

func ReadPassword(hint string) (string, error) {
	fmt.Print(hint)
	pw, err := terminal.ReadPassword(common.Stdin)
	if err != nil {
		return "", err
	}
	fmt.Println("")
	return slice.ByteToString(pw), nil
}

func ReadStdinln(hint string) (string, error) {
	fmt.Print(hint)
	reader := bufio.NewReader(os.Stdin)
	s, err := reader.ReadString('\n')
	// Trim the \r or \r\n at tail.
	s = strings.TrimRight(s, "\r\n")
	//fmt.Println("")
	if err != nil {
		return "", err
	}
	return s, nil
}
