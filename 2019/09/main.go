package main

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
)

type data struct {
	Data []int `json:"input"`
}

func checkError(e error) {
	if e != nil {
		panic(e)
	}
}

func readData(path string) []int {
	dat, e := ioutil.ReadFile(path)
	checkError(e)
	var jsonData data
	json.Unmarshal(dat, &jsonData)
	return jsonData.Data
}

func getParameter(par, mode, relative int, data []int) int {
	var param int
	if mode == 0 {
		param = data[par]
	} else if mode == 1 {
		param = par
	} else {
		param = data[par] + relative
	}
	return param
}

func runProgram(data []int, input int) {
	pos := 0
	relative := 0
	for data[pos] != 99 {
		op := data[pos]
		mod3 := (op / 10000) % 10
		mod2 := (op / 1000) % 10
		mod1 := (op / 100) % 10
		op = op % 10
		par1 := getParameter(pos+1, mod1, relative, data)
		par2 := getParameter(pos+2, mod2, relative, data)
		par3 := getParameter(pos+3, mod3, relative, data)
		switch op {
		case 1:
			data[par3] = data[par1] + data[par2]
			pos += 4
		case 2:
			data[par3] = data[par1] * data[par2]
			pos += 4
		case 3:
			data[par1] = input
			pos += 2
		case 4:
			fmt.Println("Out:", data[par1])
			pos += 2
		case 5:
			if data[par1] != 0 {
				pos = data[par2]
			} else {
				pos += 3
			}
		case 6:
			if data[par1] == 0 {
				pos = data[par2]
			} else {
				pos += 3
			}
		case 7:
			var res int
			if data[par1] < data[par2] {
				res = 1
			} else {
				res = 0
			}
			data[par3] = res
			pos += 4
		case 8:
			var res int
			if data[par1] == data[par2] {
				res = 1
			} else {
				res = 0
			}
			data[par3] = res
			pos += 4
		case 9:
			relative += data[par1]
			pos += 2
		}
	}
}

func getProgram() []int {
	data := readData("input.json")
	freeData := make([]int, 10000)
	return append(data, freeData...)
}

func first() {
	data := getProgram()
	runProgram(data, 1)
}

func second() {
	data := getProgram()
	runProgram(data, 2)
}

func main() {
	first()
	second()
}
