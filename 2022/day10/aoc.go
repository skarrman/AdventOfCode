package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

type Inst struct {
	memonic string
	val     int
}

func getSolutionPart2(input []Inst) int {
	signal_strength := 0
	ptr, cycle, register, should_add := 0, 0, 1, false
	for {
		if ptr >= len(input) {
			break
		}
		cycle += 1
		pos := (cycle - 1) % 40
		if pos-1 <= register && register <= pos+1 {
			fmt.Print("X")
		} else {
			fmt.Print(" ")
		}
		if cycle%40 == 0 {
			fmt.Println()
		}
		if cycle == 20 || (cycle-20)%40 == 0 {
			s := cycle * register
			signal_strength += s
		}
		if should_add {
			register += input[ptr].val
			should_add = false
			ptr++
		} else {
			switch input[ptr].memonic {
			case "noop":
				ptr++
			case "addx":
				should_add = true
			}
		}
	}
	return signal_strength
}

func parseInput(input string) ([]Inst, error) {
	var insts []Inst

	lines := strings.Split(input, "\n")

	for _, line := range lines {
		parts := strings.Split(line, " ")
		value := -1
		if len(parts) > 1 {
			v, err := strconv.Atoi(parts[1])
			if err != nil {
				return nil, err
			}
			value = v
		}
		insts = append(insts, Inst{parts[0], value})
	}

	return insts, nil
}

func main() {
	inputBytes, err := ioutil.ReadFile("input.txt")
	if err != nil {
		panic("couldn't read input")
	}

	input, err := parseInput(string(inputBytes))
	if err != nil {
		panic("couldn't parse input")
	}

	fmt.Println(getSolutionPart2(input))
}
