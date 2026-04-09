package main

import (
	"fmt"
	"strconv"
)

type Opcode byte

const (
	NOP Opcode = iota // identifier that starts the enum at 0
	ADD
	LOAD_CONSTANT
	PRINT
	HALT
)

// Map strings to enum values
var opcodeMap = map[string]Opcode{
	"NOP":           NOP,
	"ADD":           ADD,
	"LOAD_CONSTANT": LOAD_CONSTANT,
	"PRINT":         PRINT,
	"HALT":          HALT,
}

func convertStringToByte(s string) (byte, error) {

	n, err := strconv.Atoi(s)
	if err != nil {
		fmt.Println("Error: ", err)
		return 0, err
	}

	b := byte(n)

	return b, nil
}

func tacToByteCode(tac_instructions []Tac) []byte {

	var byteCodes []byte

	for _, instruction := range tac_instructions {
		if opcode, ok := opcodeMap[instruction.Opcode]; ok {
			switch opcode {
			case LOAD_CONSTANT:
				byteCodes = append(byteCodes, byte(opcode))
				b, err := convertStringToByte(instruction.Tacvar.Value)
				if err == nil {
					// fmt.Println("Adding: ", opcode) DEBUG
					byteCodes = append(byteCodes, b)
				} else {
					panic("Something went wrong during the LOAD_CONSTANT Conversion")
				}
			case ADD, PRINT, HALT: // might want seperate cases someday for now...
				// fmt.Println("Adding: ", opcode) DEBUG
				byteCodes = append(byteCodes, byte(opcode))

			default:
				fmt.Println("Unknown Something")
			}
		} else {
			fmt.Println("Unknown opcode:", instruction.Opcode)
		}
	}

	return byteCodes
}
