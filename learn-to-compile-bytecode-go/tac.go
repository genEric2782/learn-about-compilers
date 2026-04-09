package main

type TacExpression struct {
	Type         string `json:"$type"`
	TacTempValue string `json:"tacTempValue"`
	Value        string `json:"value"`
	Op           string `json:"op"`
	Arg1         string `json:"arg1"`
	Arg2         string `json:"arg2"`
}

type Tac struct {
	Opcode string        `json:"opcode"`
	Tacvar TacExpression `json:"tacvar"`
}
