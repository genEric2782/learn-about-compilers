package main

import (
	"encoding/json"
	"fmt"
	"os"
)

func readInTacFile() ([]Tac, error) {
	file, err := os.Open("../TacJson.json") // := is for declartion and assigments where = is jsut for assigment
	if err != nil {
		fmt.Println("Error openign file: ", err)
		return nil, err
	}
	defer file.Close()

	var tac []Tac
	jsonReader := json.NewDecoder(file)
	err = jsonReader.Decode(&tac)
	if err != nil { // nil is fancy null
		fmt.Println("Error decoding json: ", err)
		return nil, err
	}

	// data, err := json.MarshalIndent(tac, "", "  ")
	// if err != nil {
	// 	fmt.Println("Error formatting JSON:", err)
	// 	return nil, err
	// }

	// fmt.Println(string(data))

	return tac, nil
}
