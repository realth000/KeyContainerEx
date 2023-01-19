package storage

import (
	"bytes"
	"os"
	"reflect"
	"strconv"
)

func (s *Storage) Save() error {
	_, err := os.Stat(s.FilePath)
	if !os.IsNotExist(err) {
		if err = os.Remove(s.FilePath); err != nil {
			return err
		}
	}
	var dataBuffer bytes.Buffer
	dataBuffer.Write(MagicHeader)
	dataBuffer.Write(s.makeMainPasswordSection())
	dataBuffer.Write(s.makePasswordSection())
	if err = os.WriteFile(s.FilePath, dataBuffer.Bytes(), 0600); err != nil {
		return err
	}
	return nil
}

func (s *Storage) makeMainPasswordSection() []byte {
	return s.MainPassword.GetHash()
}

func (s *Storage) makePasswordSection() []byte {
	var buffer bytes.Buffer
	for _, p := range s.Password {
		if p == nil {
			break
		}
		// Get already encrypted data to store on disk.
		// Save all variable filed.
		storageData := p.StorageData()
		value := reflect.ValueOf(storageData)
		visibleFields := reflect.VisibleFields(reflect.TypeOf(storageData))
		// For every variable field in storageData, write "data length" and then following "data".
		// We write "data length" to help us know how long each field is when reading from disk.
		for _, visibleField := range visibleFields {
			var length int64
			var fieldValueBytes []byte
			fieldValue := value.FieldByName(visibleField.Name)
			switch visibleField.Type.Kind() {
			case reflect.String:
				length = int64(len(fieldValue.String()))
				fieldValueBytes = []byte(fieldValue.String())
			case reflect.Slice:
				length = int64(len(fieldValue.Bytes()))
				fieldValueBytes = fieldValue.Bytes()
			default:
				continue
			}
			buffer.Write([]byte(strconv.FormatInt(length, 10)))
			buffer.Write(fieldValueBytes)
		}
	}
	return buffer.Bytes()
}
