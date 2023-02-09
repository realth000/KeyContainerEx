package core

import (
	"KeyContainerEx/storage"
	"fmt"
)

func RemovePassword(storage *storage.Storage, comment string) error {
	if storage == nil {
		return fmt.Errorf("nil storage")
	}
	for k, v := range storage.Password {
		if c, err := v.Comment(); err != nil {
			return fmt.Errorf("failed to get comment: %w", err)
		} else if c == comment {
			delete(storage.Password, k)
			break
		}
	}
	return nil
}
