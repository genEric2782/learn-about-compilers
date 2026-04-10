package main

import "fmt"

type VM struct {
	ip       int    // instruction pointer
	stack    *Stack // operand stack
	bytecode []byte // bytecode
}

func NewVM(bytecode []byte) *VM {
	return &VM{
		ip:       0,
		stack:    GlobalStack,
		bytecode: bytecode,
	}
}

func (vm *VM) Run() {
	for vm.ip < len(vm.bytecode) {
		// TODO: This seesm really fragile maybe i should make a tuple or something with the bytecode and what the code is as extra
		// guarentee could also add the temp var number (t1, t2, t3)
		// for instructions like add so i dont just double pop i actually pop the specific instruction?

		opcode := Opcode(vm.bytecode[vm.ip]) // need to cast the ytes as opcodes or seitch gets made, works since opcode are of type byte
		vm.ip++

		switch opcode {
		case LOAD_CONSTANT:
			constant := byte(vm.bytecode[vm.ip])
			vm.ip++
			vm.stack.Push(constant)
		case ADD:
			val2, ok := vm.stack.Pop()
			val1, ok2 := vm.stack.Pop()
			if !ok && !ok2 {
				panic("Something went wrong during pop operation")
			}
			byteVal, ok := val1.(byte)
			if !ok {
				panic("Value 1 was not a byte")
			}
			byteVal2, ok2 := val2.(byte)
			if !ok {
				panic("Value 2 was not a byte")
			}
			intVal := int(byteVal)
			intVal2 := int(byteVal2)

			// ADD doesnt do the vm.ip++ becuase ip get incremented before switch and Add doesnt add onto the stack like LOAD_CONSTANT does
			// or there is no argument in the bytecode list for add since its adding 2 things from the stack
			vm.stack.Push(intVal + intVal2)
		case MINUS:
			val2, ok := vm.stack.Pop()
			val1, ok2 := vm.stack.Pop()
			if !ok && !ok2 {
				panic("Something went wrong during pop operation")
			}
			byteVal, ok := val1.(byte)
			if !ok {
				panic("Value 1 was not a byte")
			}
			byteVal2, ok2 := val2.(byte)
			if !ok {
				panic("Value 2 was not a byte")
			}
			intVal := int(byteVal)
			intVal2 := int(byteVal2)

			// SUB doesnt do the vm.ip++ becuase ip get incremented before switch and Add doesnt add onto the stack like LOAD_CONSTANT does
			// or there is no argument in the bytecode list for add since its adding 2 things from the stack
			vm.stack.Push(intVal - intVal2)
		case MULTIPLY:
			val2, ok := vm.stack.Pop()
			val1, ok2 := vm.stack.Pop()
			if !ok && !ok2 {
				panic("Something went wrong during pop operation")
			}
			byteVal, ok := val1.(byte)
			if !ok {
				panic("Value 1 was not a byte")
			}
			byteVal2, ok2 := val2.(byte)
			if !ok {
				panic("Value 2 was not a byte")
			}
			intVal := int(byteVal)
			intVal2 := int(byteVal2)

			vm.stack.Push(intVal * intVal2)
		case DIVIDE:
			val2, ok := vm.stack.Pop()
			val1, ok2 := vm.stack.Pop()
			if !ok && !ok2 {
				panic("Something went wrong during pop operation")
			}
			byteVal, ok := val1.(byte)
			if !ok {
				panic("Value 1 was not a byte")
			}
			byteVal2, ok2 := val2.(byte)
			if !ok {
				panic("Value 2 was not a byte")
			}
			intVal := int(byteVal)
			intVal2 := int(byteVal2)

			vm.stack.Push(intVal / intVal2)
		case PRINT:
			// TODO: this needs to be way more dynamic can't just assume the value at the top of the stack is the value i want to print
			val, ok := vm.stack.Pop()
			if !ok {
				panic("Something went wrong during pop operation")
			}
			fmt.Println("The Value of the operation: ", val)
		case HALT:
			return

		default:
			panic(fmt.Sprintf("unknown opcode %d", opcode))
		}
	}
}
