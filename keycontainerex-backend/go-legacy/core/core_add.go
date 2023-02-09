package core

import (
	"KeyContainerEx/secure"
	"KeyContainerEx/storage"
)

func AddPassword(activeStorage *storage.Storage, account, password, comment string, option *secure.PasswordOption) error {
	pw, err := secure.NewPassword(account, password, comment, *option)
	if err != nil {
		return err
	}
	if err = activeStorage.AddPassword(pw); err != nil {
		return err
	}
	return activeStorage.Save()
}
