package main

/*
#include <stdlib.h>
*/
import "C"

import (
	"encoding/json"
	"fmt"
)

//export ReadTacFromFfi
func ReadTacFromFfi(input *C.char) {

	tacJsonStr := C.GoString(input)

	var tac []Tac
	err := json.Unmarshal([]byte(tacJsonStr), &tac)
	if err != nil {
		fmt.Println("Error Parsing Json: ", err)
		return
	}

	for i, entry := range tac {
		fmt.Println("Instruction", i, "Opcode:", entry.Opcode)

		// Just print raw tacvar for now
		fmt.Println("Raw tacvar:", entry.Tacvar)
	}

	// Post proocessing
	// byte_codes := tacToByteCode(tac)

	// TODO For now we are going to add a PRINT and a HALT to this byte_code
	// will need to add these to tac in future but as these are not part of the asm code itll get a little tricky....
	tac = append(tac, Tac{Opcode: "PRINT"})
	tac = append(tac, Tac{Opcode: "HALT"})

	vm := NewVM(tacToByteCode(tac))
	vm.Run()
}
