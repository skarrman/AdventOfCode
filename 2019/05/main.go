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

func getParameter(par int, mode int, data []int) int {
	if par >= len(data) {
		return 0
	}
	var param int
	if mode == 0 {
		param = data[data[par]]
	} else {
		param = data[par]
	}
	return param
}

func runProgram(data []int, input int) {
	pos := 0
	for {
		op := data[pos]
		var mod1 int
		var mod2 int
		if op == 99 {
			break
		}
		if op > 10 {
			mod2 = (op / 1000) % 10
			mod1 = (op / 100) % 10
			op = op % 10
		} else {
			mod2 = 0
			mod1 = 0
		}

		if op == 1 {
			data[data[pos+3]] = getParameter(pos+1, mod1, data) + getParameter(pos+2, mod2, data)
			pos += 4
		} else if op == 2 {
			data[data[pos+3]] = getParameter(pos+1, mod1, data) * getParameter(pos+2, mod2, data)
			pos += 4
		} else if op == 3 {
			data[data[pos+1]] = input
			pos += 2
		} else if op == 4 {
			fmt.Println(getParameter(pos+1, mod1, data))
			pos += 2
		} else if op == 5 {
			if getParameter(pos+1, mod1, data) != 0 {
				pos = getParameter(pos+2, mod2, data)
			} else {
				pos += 3
			}
		} else if op == 6 {
			if getParameter(pos+1, mod1, data) == 0 {
				pos = getParameter(pos+2, mod2, data)
			} else {
				pos += 3
			}
		} else if op == 7 {
			var res int
			if getParameter(pos+1, mod1, data) < getParameter(pos+2, mod2, data) {
				res = 1
			} else {
				res = 0
			}
			data[data[pos+3]] = res
			pos += 4
		} else if op == 8 {
			var res int
			if getParameter(pos+1, mod1, data) == getParameter(pos+2, mod2, data) {
				res = 1
			} else {
				res = 0
			}
			data[data[pos+3]] = res
			pos += 4
		}
	}
}

func first() {
	data := readData("input.json")
	runProgram(data, 1)
}

func second() {
	data := readData("input.json")
	runProgram(data, 5)
}

func main() {
	first()
	second()
}
