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

func heapPermutation(a []int, size int, seqs *[][]int) {
	if size == 1 {
		l := make([]int, len(a))
		copy(l, a)
		*seqs = append(*seqs, l)
	}

	for i := 0; i < size; i++ {
		heapPermutation(a, size-1, seqs)

		if size%2 == 1 {
			a[0], a[size-1] = a[size-1], a[0]
		} else {
			a[i], a[size-1] = a[size-1], a[i]
		}
	}
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

func runProgram(pos int, data []int, phase, input int, first bool) (int, int, int) {
	output := -1
	for data[pos] != 99 {
		op := data[pos]
		var mod1 int
		var mod2 int
		mod2 = (op / 1000) % 10
		mod1 = (op / 100) % 10
		op = op % 10

		if op == 1 {
			data[data[pos+3]] = getParameter(pos+1, mod1, data) + getParameter(pos+2, mod2, data)
			pos += 4
		} else if op == 2 {
			data[data[pos+3]] = getParameter(pos+1, mod1, data) * getParameter(pos+2, mod2, data)
			pos += 4
		} else if op == 3 {
			var in int
			if first {
				in = phase
			} else {
				in = input
			}
			first = false
			data[data[pos+1]] = in
			pos += 2
		} else if op == 4 {
			output = getParameter(pos+1, mod1, data)
			pos += 2
			return pos, output, op
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
	return pos, output, 99
}

func copyProgram(data []int) []int {
	new := make([]int, len(data))
	copy(new, data)
	return new
}

func second() {
	data := readData("input.json")
	var amps [][]int
	for i := 0; i < 5; i++ {
		amps = append(amps, copyProgram(data))
	}
	var seqs [][]int
	max := -1
	heapPermutation([]int{5, 6, 7, 8, 9}, 5, &seqs)
	for _, seq := range seqs {
		poss := []int{0, 0, 0, 0, 0}
		out := 0
		first := true
		op := 0
		for op != 99 {
			for amp := 0; amp < 5; amp++ {
				var output int
				poss[amp], output, op = runProgram(poss[amp], amps[amp], seq[amp], out, first)
				if op != 99 {
					out = output
				}
			}
			first = false
		}
		if out > max {
			max = out
		}
	}

	fmt.Println(max)
}

func first() {
	data := readData("input.json")
	var seqs [][]int
	heapPermutation([]int{0, 1, 2, 3, 4}, 5, &seqs)
	max := -1
	for _, seq := range seqs {
		out := 0
		for _, s := range seq {
			d := copyProgram(data)
			_, out, _ = runProgram(0, d, s, out, true)
		}
		if out > max {
			max = out
		}
	}
	fmt.Println(max)
}

func main() {
	first()
	second()
}
