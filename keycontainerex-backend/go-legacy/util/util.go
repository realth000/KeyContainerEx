package util

import (
	"bufio"
	"fmt"
	"github.com/realth000/ToGoTool/slice"
	"golang.org/x/crypto/ssh/terminal"
	"math/rand"
	"os"
	"strings"
	"time"
)

func init() {
	rand.Seed(time.Now().UnixNano())
}

var letterRunes = []rune("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ")

func ReadPassword(hint string) (string, error) {
	fmt.Print(hint)
	pw, err := terminal.ReadPassword(int(os.Stdin.Fd()))
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

func RandHashString(n int) string {
	b := make([]rune, n)
	for i := range b {
		b[i] = letterRunes[rand.Intn(len(letterRunes))]
	}
	return string(b)
}
