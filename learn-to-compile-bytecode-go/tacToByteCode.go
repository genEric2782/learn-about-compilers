package main

import (
	"fmt"
	"strconv"
)

type Opcode byte

const (
	NOP Opcode = iota // identifier that starts the enum at 0
	ADD
	MINUS
	MULTIPLY
	DIVIDE
	LOAD_CONSTANT
	PRINT
	HALT
)

// Map strings to enum values
var opcodeMap = map[string]Opcode{
	"NOP":           NOP,
	"ADD":           ADD,
	"MINUS":         MINUS,
	"MULTIPLY":      MULTIPLY,
	"DIVIDE":        DIVIDE,
	"LOAD_CONSTANT": LOAD_CONSTANT,
	"PRINT":         PRINT,
	"HALT":          HALT,
}

func convertStringToInt(s string) (int, error) {

	// n, err := strconv.Atoi(s)
	// if err != nil {
	// 	fmt.Println("Error: ", err)
	// 	return 0, err
	// }

	// b := byte(n)

	return strconv.Atoi(s)
}

func tacToByteCode(tac_instructions []Tac) []Instruction {

	var instructions []Instruction

	for _, instruction := range tac_instructions {
		if opcode, ok := opcodeMap[instruction.Opcode]; ok {
			switch opcode {
			case LOAD_CONSTANT:
				// byteCodes = append(byteCodes, byte(opcode))
				b, err := convertStringToInt(instruction.Tacvar.Value)
				if err != nil {
					// fmt.Println("Adding: ", opcode) DEBUG
					panic("Something went wrong during the LOAD_CONSTANT Conversion")
				}
				instructions = append(instructions, Instruction{
					Op:   opcode,
					data: []Value{Value(b)},
				})
			case ADD, MINUS, MULTIPLY, DIVIDE, PRINT, HALT: // might want seperate cases someday for now...
				// fmt.Println("Adding: ", opcode) DEBUG
				instructions = append(instructions, Instruction{
					Op:   opcode,
					data: nil,
				})

			default:
				fmt.Println("Unknown Something")
			}
		} else {
			fmt.Println("Unknown opcode:", instruction.Opcode)
		}
	}

	return instructions
}
