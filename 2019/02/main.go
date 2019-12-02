package main

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
)

type Data struct {
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
	var jsonData Data
	json.Unmarshal(dat, &jsonData)
	return jsonData.Data
}

func runProgram(data []int) {
	pos := 0
	for {
		if data[pos] == 1 {
			data[data[pos+3]] = data[data[pos+1]] + data[data[pos+2]]
		} else if data[pos] == 2 {
			data[data[pos+3]] = data[data[pos+1]] * data[data[pos+2]]
		} else {
			break
		}
		pos += 4
	}
}

func first() {
	data := readData("input.json")
	data[1] = 12
	data[2] = 2
	runProgram(data)
	fmt.Println(data[0])
}

func copyData(l []int) []int {
	l2 := make([]int, len(l))
	copy(l2, l)
	return l2
}

func second() {
	goal := 19690720
	data := readData("input.json")
	for noun := 0; noun < 100; noun++ {
		for verb := 0; verb < 100; verb++ {
			curData := copyData(data)
			curData[1] = noun
			curData[2] = verb
			runProgram(curData)
			if curData[0] == goal {
				fmt.Println(100*noun + verb)
				break
			}
		}
	}
}

func main() {
	first()
	second()
}
