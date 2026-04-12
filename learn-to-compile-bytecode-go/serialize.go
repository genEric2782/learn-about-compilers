package main

import (
	"encoding/binary"
	"fmt"
	"os"
)

const MagicBytes = 0x4C544350 // "LTCP"
const Version = 1

func Serialize(instructions []Instruction, path string) error {
	f, err := os.Create(path)
	if err != nil {
		return err
	}

	defer f.Close()

	/* Basic foramt for binary sturcture
	4 bytes magic number
	4 bytes version
	4 bytes number of instructions
	*/
	binary.Write(f, binary.LittleEndian, uint32(MagicBytes))
	binary.Write(f, binary.LittleEndian, uint32(Version))
	binary.Write(f, binary.LittleEndian, uint32(len(instructions)))

	// Serialize instructions
	/* Layout
	1 byte opcode
	1 byte is_operand
	8 bytes operand values
	*/
	for _, instruction := range instructions {

		binary.Write(f, binary.LittleEndian, uint8(instruction.Op))
		if len(instruction.data) > 0 {
			binary.Write(f, binary.LittleEndian, uint8(1))
			binary.Write(f, binary.LittleEndian, int64(instruction.data[0].(int)))
		} else { // 0 indicating no data
			binary.Write(f, binary.LittleEndian, uint8(0))
			binary.Write(f, binary.LittleEndian, int64(0))
		}
	}
	return nil
}
func Deserialize(path string) ([]Instruction, error) {
	f, err := os.Open(path)
	if err != nil {
		return nil, err
	}
	defer f.Close()
	// file validation
	var magic_bytes, version, instruction_count uint32
	binary.Read(f, binary.LittleEndian, &magic_bytes)
	if magic_bytes != MagicBytes {
		return nil, fmt.Errorf("Invalid file: invalid magic ")
	}
	binary.Read(f, binary.LittleEndian, &version)
	binary.Read(f, binary.LittleEndian, &instruction_count)

	// TODO explain make
	instructions := make([]Instruction, instruction_count)
	for i := range instructions {
		var op uint8
		var has_operand uint8
		var operand int64

		binary.Read(f, binary.LittleEndian, &op)
		binary.Read(f, binary.LittleEndian, &has_operand)
		binary.Read(f, binary.LittleEndian, &operand)

		intruction := Instruction{Op: Opcode(op)}
		if has_operand == 1 {
			intruction.data = []Value{int(operand)}
		}
		instructions[i] = intruction
	}
	return instructions, nil
}
