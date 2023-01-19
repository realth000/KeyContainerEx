package storage

import (
	"bytes"
	"encoding/binary"
	"os"
	"reflect"
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
	dataBuffer.WriteByte(version)
	dataBuffer.Write(s.makeMainPasswordSection())
	dataBuffer.Write(s.makePasswordSection())
	if err = os.WriteFile(s.FilePath, dataBuffer.Bytes(), 0600); err != nil {
		return err
	}
	return nil
}

func (s *Storage) makeMainPasswordSection() []byte {
	buffer := new(bytes.Buffer)
	hashValue := s.MainPassword.GetHash()
	//buffer.WriteString(strconv.FormatInt(int64(s.MainPassword.HashType), 10))
	_ = binary.Write(buffer, binary.LittleEndian, int8(s.MainPassword.HashType))
	_ = binary.Write(buffer, binary.LittleEndian, uint32(len(hashValue)))
	buffer.Write(hashValue)
	buffer.WriteByte(mainPasswordSplit)
	return buffer.Bytes()
}

func (s *Storage) makePasswordSection() []byte {
	buffer := new(bytes.Buffer)
	for _, p := range s.Password {
		if p == nil {
			break
		}
		// Write crypto type.
		_ = binary.Write(buffer, binary.LittleEndian, int8(p.Option.AESType))
		// Write crypto mode.
		_ = binary.Write(buffer, binary.LittleEndian, int8(p.Option.AESMode))

		// Get already encrypted data to store on disk.
		// Save all variable filed.
		storageData := p.StorageData()
		value := reflect.ValueOf(storageData)
		visibleFields := reflect.VisibleFields(reflect.TypeOf(storageData))
		// For every variable field in storageData, write "data length" and then following "data".
		// We write "data length" to help us know how long each field is when reading from disk.
		for _, visibleField := range visibleFields {
			var length uint32
			var fieldValueBytes []byte
			fieldValue := value.FieldByName(visibleField.Name)
			switch visibleField.Type.Kind() {
			// Now we only have byte slice member.
			// case reflect.String:
			// 	length = uint32(len(fieldValue.String()))
			// 	fieldValueBytes = []byte(fieldValue.String())
			case reflect.Slice:
				length = uint32(len(fieldValue.Bytes()))
				fieldValueBytes = fieldValue.Bytes()
			default:
				continue
			}
			_ = binary.Write(buffer, binary.LittleEndian, uint32(length))
			buffer.Write(fieldValueBytes)
		}
		buffer.WriteByte(passwordSplit)
	}
	return buffer.Bytes()
}
