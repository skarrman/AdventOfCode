package main

import (
	"fmt"
	"io/ioutil"
	"os"
	"sort"
	"strings"
)

func checkRows(input [][]string) (int, int) {
	openChars := map[string]bool{"(": true, "[": true, "{": true, "<": true}
	closeChars := map[string]string{"(": ")", "[": "]", "{": "}", "<": ">"}
	syntaxPoints := map[string]int{")": 3, "]": 57, "}": 1197, ">": 25137}
	completionPoints := map[string]int{")": 1, "]": 2, "}": 3, ">": 4}
	syntaxScore := 0
	var completionScores []int
	for _, row := range input {
		var stack []string
		incomplete := true
		for _, char := range row {
			if openChars[char] {
				stack = append(stack, char)
			} else {
				head := stack[len(stack)-1]
				stack = stack[:len(stack)-1]
				if closeChars[head] != char {
					syntaxScore += syntaxPoints[char]
					incomplete = false
					break
				}
			}
		}
		if incomplete && len(stack) > 0 {
			score := 0
			for i, _ := range stack {
				score = score*5 + completionPoints[closeChars[stack[len(stack)-1-i]]]
			}
			completionScores = append(completionScores, score)
		}
	}
	sort.Ints(completionScores)
	return syntaxScore, completionScores[len(completionScores)/2]
}

func parseInput(input string) [][]string {
	var lines [][]string
	for _, line := range strings.Split(strings.TrimSpace(input), "\n") {
		var chars []string
		for _, char := range strings.Split(line, "") {
			chars = append(chars, char)
		}
		lines = append(lines, chars)
	}
	return lines
}

func main() {
	inputBytes, err := ioutil.ReadFile("input.txt")
	if err != nil {
		panic("couldn't read input")
	}
	part1, part2 := checkRows(parseInput(string(inputBytes)))
	if os.Getenv("part") == "part2" {
		fmt.Println(part2)
	} else {
		fmt.Println(part1)
	}
}
