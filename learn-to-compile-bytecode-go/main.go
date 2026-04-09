package main

import "fmt"

func main() {
	// fmt.Println("Hello, Go Project!")
	// initalize Global stack

	tac, err := readInTacFile()
	if err != nil {
		fmt.Println("Error when reading in the tac:", err)
		return
	}

	byte_codes := tacToByteCode(tac)

	// will need to add these to tac in future but as these are not part of the asm code itll get a little tricky....
	byte_codes = append(byte_codes, byte(PRINT))
	byte_codes = append(byte_codes, byte(HALT))

	vm := NewVM(byte_codes)
	vm.Run()

	// // Sanity check
	// fmt.Println("Tac Data: ", tac)
}
