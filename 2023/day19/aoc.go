package main

import (
	"errors"
	"fmt"
	"io/ioutil"
	"os"
	"regexp"
	"strconv"
	"strings"
)

var (
	ruleReg      = regexp.MustCompile(`(?m)([^{]+)\{(.)(<|>)([^:]+):([^,]+),([^}]+)\}`)
	innerRuleReg = regexp.MustCompile(`(?m)(.)(<|>)([^:]+):([^,]+),([^}]+)`)
	varReg       = regexp.MustCompile(`(?m)\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}`)
	varMap       = map[string]int{"x": 0, "m": 1, "a": 2, "s": 3}
)

const (
	vari = 0
	op   = 1
	val  = 2
	t    = 3
	f    = 4
)

func getSolutionPart1(vars [][]int, rules map[string][]string) int {
	res := 0
	for _, part := range vars {
		curKey := "in"
		for curKey != "R" && curKey != "A" {
			rule, was := rules[curKey]

			if !was {
				rule = innerRuleReg.FindStringSubmatch(curKey)[1:]
			} else {
			}
			if len(rule) != 5 {
				return -1
			}
			v, err := strconv.Atoi(rule[val])
			if err != nil {
				return -1
			}
			switch rule[op] {
			case "<":
				if part[varMap[rule[vari]]] < v {
					curKey = rule[t]
				} else {
					curKey = rule[f]
				}
			case ">":
				if part[varMap[rule[vari]]] > v {
					curKey = rule[t]
				} else {
					curKey = rule[f]
				}
			}
		}
		if curKey == "A" {
			for _, v := range part {
				res += v
			}
		}
	}
	return res
}

type ranges struct {
	minX int
	maxX int
	minM int
	maxM int
	minA int
	maxA int
	minS int
	maxS int
}

func initRanges() ranges {
	return ranges{minX: 1, maxX: 4000, minM: 1, maxM: 4000, minA: 1, maxA: 4000, minS: 1, maxS: 4000}
}

func copyRanges(r ranges) ranges {
	return ranges{minX: r.minX, maxX: r.maxX, minM: r.minM, maxM: r.maxM, minA: r.minA, maxA: r.maxA, minS: r.minS, maxS: r.maxS}
}

func isValid(ranges ranges) bool {
	return ranges.minX < ranges.maxX && ranges.minM < ranges.maxM && ranges.minA < ranges.maxA && ranges.minS < ranges.maxS
}

type entry struct {
	rule   string
	ranges ranges
}

func getSolutionPart2(rules map[string][]string) int {
	states := []entry{{rule: "in", ranges: initRanges()}}
	var validRanges []ranges
	for len(states) > 0 {
		state := states[0]
		states = states[1:]
		if !isValid(state.ranges) {
			continue
		}
		rule, is := rules[state.rule]
		if !is {
			switch state.rule {
			case "A":
				validRanges = append(validRanges, state.ranges)
				continue
			case "R":
				continue
			default:
				rule = innerRuleReg.FindStringSubmatch(state.rule)[1:]
			}
		}

		v, _ := strconv.Atoi(rule[val])

		switch rule[vari] {
		case "x":
			if rule[op] == "<" {
				tnext := copyRanges(state.ranges)
				tnext.maxX = v - 1
				states = append(states, entry{rule: rule[t], ranges: tnext})
				fnext := copyRanges(state.ranges)
				fnext.minX = v
				states = append(states, entry{rule: rule[f], ranges: fnext})
			} else {
				tnext := copyRanges(state.ranges)
				tnext.minX = v + 1
				states = append(states, entry{rule: rule[t], ranges: tnext})
				fnext := copyRanges(state.ranges)
				fnext.maxX = v
				states = append(states, entry{rule: rule[f], ranges: fnext})
			}
		case "m":
			if rule[op] == "<" {
				tnext := copyRanges(state.ranges)
				tnext.maxM = v - 1
				states = append(states, entry{rule: rule[t], ranges: tnext})
				fnext := copyRanges(state.ranges)
				fnext.minM = v
				states = append(states, entry{rule: rule[f], ranges: fnext})
			} else {
				tnext := copyRanges(state.ranges)
				tnext.minM = v + 1
				states = append(states, entry{rule: rule[t], ranges: tnext})
				fnext := copyRanges(state.ranges)
				fnext.maxM = v
				states = append(states, entry{rule: rule[f], ranges: fnext})
			}
		case "a":
			if rule[op] == "<" {
				tnext := copyRanges(state.ranges)
				tnext.maxA = v - 1
				states = append(states, entry{rule: rule[t], ranges: tnext})
				fnext := copyRanges(state.ranges)
				fnext.minA = v
				states = append(states, entry{rule: rule[f], ranges: fnext})
			} else {
				tnext := copyRanges(state.ranges)
				tnext.minA = v + 1
				states = append(states, entry{rule: rule[t], ranges: tnext})
				fnext := copyRanges(state.ranges)
				fnext.maxA = v
				states = append(states, entry{rule: rule[f], ranges: fnext})
			}
		case "s":
			if rule[op] == "<" {
				tnext := copyRanges(state.ranges)
				tnext.maxS = v - 1
				states = append(states, entry{rule: rule[t], ranges: tnext})
				fnext := copyRanges(state.ranges)
				fnext.minS = v
				states = append(states, entry{rule: rule[f], ranges: fnext})
			} else {
				tnext := copyRanges(state.ranges)
				tnext.minS = v + 1
				states = append(states, entry{rule: rule[t], ranges: tnext})
				fnext := copyRanges(state.ranges)
				fnext.maxS = v
				states = append(states, entry{rule: rule[f], ranges: fnext})
			}
		}

	}

	combinations := 0
	for _, ranges := range validRanges {
		combinations += (ranges.maxX - ranges.minX + 1) * (ranges.maxM - ranges.minM + 1) * (ranges.maxA - ranges.minA + 1) * (ranges.maxS - ranges.minS + 1)
	}

	return combinations
}

func parseInput(input string) (map[string][]string, [][]int, error) {
	rules := make(map[string][]string)
	var vars [][]int

	parts := strings.Split(strings.TrimSpace(input), "\n\n")
	ruleLines := strings.Split(parts[0], "\n")
	for _, line := range ruleLines {
		m := ruleReg.FindStringSubmatch(line)
		rules[m[1]] = m[2:]
	}
	varLines := strings.Split(parts[1], "\n")
	for _, line := range varLines {
		m := varReg.FindStringSubmatch(line)
		var v []int
		if len(m) != 5 {
			return nil, nil, errors.New("Var error")
		}
		for _, ma := range m[1:] {
			i, err := strconv.Atoi(ma)
			if err != nil {
				return nil, nil, err
			}
			v = append(v, i)
		}
		vars = append(vars, v)
	}
	return rules, vars, nil
}

func main() {
	inputBytes, err := ioutil.ReadFile("input.txt")
	if err != nil {
		panic("couldn't read input")
	}

	rules, vars, err := parseInput(string(inputBytes))
	if err != nil {
		panic("couldn't parse input")
	}

	part := os.Getenv("part")

	if part == "part2" {
		fmt.Println(getSolutionPart2(rules))
	} else {
		fmt.Println(getSolutionPart1(vars, rules))
	}
}
