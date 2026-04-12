package main

import "fmt"

type Value interface{}
type Instruction struct {
	Op Opcode
	// TempVar string TOOD: this way the stack will know exactly what its holding will make things like add more robust
	// Args    []string
	data []Value
}

type VM struct {
	ip               int           // instruction pointer
	stack            *Stack        // operand stack
	instruction_data []Instruction // bytecode
}

func NewVM(instructions []Instruction) *VM {
	return &VM{
		ip:               0,
		stack:            GlobalStack,
		instruction_data: instructions,
	}
}

func (vm *VM) popValueHelper() Value {
	val, ok := vm.stack.Pop()
	if !ok {
		panic("Something went wrong during pop operation")
	}
	// unwrap the value from type interface{}
	v, ok := val.(Value)
	if !ok {
		panic(fmt.Sprintf("expected Value on stack, got %T", val))
	}
	return v
}

func (vm *VM) Run() {
	for vm.ip < len(vm.instruction_data) {
		// TODO: This seesm really fragile maybe i should make a tuple or something with the bytecode and what the code is as extra
		// guarentee could also add the temp var number (t1, t2, t3)
		// for instructions like add so i dont just double pop i actually pop the specific instruction?

		opcode := vm.instruction_data[vm.ip].Op // need to cast the ytes as opcodes or seitch gets made, works since opcode are of type byte
		vm.ip++

		switch opcode {
		case LOAD_CONSTANT:
			constant := vm.instruction_data[vm.ip-1].data[0]
			vm.stack.Push(constant)
		case ADD:
			v2 := vm.popValueHelper()
			v1 := vm.popValueHelper()

			n1, ok1 := v1.(int)
			n2, ok2 := v2.(int)
			if !ok1 || !ok2 {
				panic(fmt.Sprintf("ADD expected int values, got %T and %T", v1, v2))
			}
			vm.stack.Push(n1 + n2)
		case MINUS:
			v2 := vm.popValueHelper()
			v1 := vm.popValueHelper()

			// TODO Can still do better this assume will be given ints
			// will need to update when i add things like floats
			n1, ok1 := v1.(int)
			n2, ok2 := v2.(int)
			if !ok1 || !ok2 {
				panic(fmt.Sprintf("SUB expected int values, got %T and %T", v1, v2))
			}

			vm.stack.Push(n1 - n2)
		case MULTIPLY:
			v2 := vm.popValueHelper()
			v1 := vm.popValueHelper()

			n1, ok1 := v1.(int)
			n2, ok2 := v2.(int)
			if !ok1 || !ok2 {
				panic(fmt.Sprintf("MUL expected int values, got %T and %T", v1, v2))
			}

			vm.stack.Push(n1 * n2)
		case DIVIDE:
			v2 := vm.popValueHelper()
			v1 := vm.popValueHelper()

			n1, ok1 := v1.(int)
			n2, ok2 := v2.(int)
			if !ok1 || !ok2 {
				panic(fmt.Sprintf("DIV expected int values, got %T and %T", v1, v2))
			}

			vm.stack.Push(n1 / n2)
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
