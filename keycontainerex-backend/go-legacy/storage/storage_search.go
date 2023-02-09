package storage

import (
	"KeyContainerEx/secure"
	"fmt"
	"github.com/gobwas/glob"
	_ "github.com/gobwas/glob"
	"regexp"
	"strings"
)

func (s *Storage) Search(text string, searchAccount, searchPassword, searchComment bool) ([]*secure.Password, error) {
	// TODO: This maybe unsafe.
	if !strings.HasPrefix(text, "*") && !strings.HasPrefix(text, "*") {
		text = fmt.Sprintf("*%s*", text)
	}
	globExp, err := glob.Compile(text)
	if err != nil {
		return nil, fmt.Errorf("invalid search glob pattern: %w", err)
	}
	var ret []*secure.Password
	for _, j := range s.Password {
		if searchAccount {
			a, err := j.Account()
			if err != nil {
				return nil, err
			}
			if globExp.Match(a) {
				ret = append(ret, j)
				continue
			}
		}
		if searchPassword {
			p, err := j.Password()
			if err != nil {
				return nil, err
			}
			if globExp.Match(p) {
				ret = append(ret, j)
				continue
			}
		}
		if searchComment {
			c, err := j.Comment()
			if err != nil {
				return nil, err
			}
			if globExp.Match(c) {
				ret = append(ret, j)
				continue
			}
		}
	}
	return ret, nil
}

func (s *Storage) SearchRegexp(reg string, searchAccount, searchPassword, searchComment bool) ([]*secure.Password, error) {
	re, err := regexp.Compile(reg)
	if err != nil {
		return nil, fmt.Errorf("invalid search glob pattern: %w", err)
	}
	var ret []*secure.Password
	for _, j := range s.Password {
		if searchAccount {
			a, err := j.Account()
			if err != nil {
				return nil, err
			}
			if re.MatchString(a) {
				ret = append(ret, j)
				continue
			}
		}
		if searchPassword {
			p, err := j.Password()
			if err != nil {
				return nil, err
			}
			if re.MatchString(p) {
				ret = append(ret, j)
				continue
			}
		}
		if searchComment {
			c, err := j.Comment()
			if err != nil {
				return nil, err
			}
			if re.MatchString(c) {
				ret = append(ret, j)
				continue
			}
		}
	}
	return ret, nil
}
