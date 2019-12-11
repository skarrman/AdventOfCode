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

func runProgram(data []int, input, pos, relative int) (int, int, int, int) {
	first := true
	color := -1
	dir := -1
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
			if first {
				color = data[par1]
				first = false
			} else {
				dir = data[par1]
				return pos + 2, color, dir, relative
			}
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
		default:
			panic(fmt.Sprintf("%s%d%s", "Opcode: ", op, " not reconized"))
		}
	}
	return pos, -1, -1, relative
}
func getProgram() []int {
	data := readData("input.json")
	freeData := make([]int, 10000)
	return append(data, freeData...)
}

type point struct {
	x, y int
}

func getDir(curr point, val int) point {
	var newPoint point
	switch curr {
	case point{0, 1}:
		newPoint = point{-1, 0}
	case point{0, -1}:
		newPoint = point{1, 0}
	case point{1, 0}:
		newPoint = point{0, 1}
	case point{-1, 0}:
		newPoint = point{0, -1}
	}
	switch val {
	case 0:
		return newPoint
	case 1:
		return point{newPoint.x * -1, newPoint.y * -1}
	default:
		panic(fmt.Sprintf("%s%d", "Dir val not expexted: ", val))
	}
}

func printReg(paintings map[point]int) {
	maxX, minX, maxY, minY := 0, 0, 0, 0
	for p := range paintings {
		if p.x > maxX {
			maxX = p.x
		}
		if p.x < minX {
			minX = p.x
		}
		if p.y > maxY {
			maxY = p.y
		}
		if p.y < minY {
			minY = p.y
		}
	}
	normPaint := make(map[point]int)
	for p, v := range paintings {
		normPaint[point{p.x - minX, p.y - minY}] = v
	}
	maxX -= minX
	maxY -= minY
	for y := 0; y <= maxY; y++ {
		for x := 0; x <= maxX; x++ {
			if v, is := normPaint[point{x, y}]; !is || v == 0 {
				fmt.Print(" ")
			} else {
				fmt.Print("#")
			}
		}
		fmt.Println()
	}
}

func run(second bool) {
	data := getProgram()
	paintings := make(map[point]int)
	currPoint := point{0, 0}
	currDir := point{0, 1}
	pos := 0
	input := 0
	relative := 0
	if second {
		paintings[currPoint] = 1
	}
	for {
		if val, is := paintings[currPoint]; is {
			input = val
		} else {
			input = 0
		}
		color, dir := 0, 0
		pos, color, dir, relative = runProgram(data, input, pos, relative)
		if data[pos] == 99 {
			break
		}
		paintings[currPoint] = color
		currDir = getDir(currDir, dir)
		currPoint = point{currPoint.x + currDir.x, currPoint.y + currDir.y}
	}
	if !second {
		fmt.Println(len(paintings))
	} else {
		printReg(paintings)
	}
}

func main() {
	run(false)
	run(true)
}
