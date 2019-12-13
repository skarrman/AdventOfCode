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

type tile struct {
	x, y, id int
}

type program struct {
	pos, relative int
	data          []int
}

func getTiles(data []int, input int) []tile {
	pos := 0
	relative := 0
	count := 0
	var current tile
	var tiles []tile
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
			data[par1] = -1
			pos += 2
		case 4:
			out := data[par1]
			switch count {
			case 0:
				current.x = out
				count++
				break
			case 1:
				current.y = out
				count++
				break
			case 2:
				current.id = out
				tiles = append(tiles, current)
				count = 0
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
		}
	}
	return tiles
}

func getProgram() []int {
	data := readData("input.json")
	freeData := make([]int, 10000)
	return append(data, freeData...)
}

func runGame(data []int, input int, paddle, ball point, board [][]int) int {
	score := 0
	pos := 0
	relative := 0
	var current tile
	count := 0
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
			if ball.y < paddle.y {
				input = -1
			} else if ball.y > paddle.y {
				input = 1
			} else {
				input = 0
			}
			fmt.Println(input, paddle, ball)
			printBoard(board)
			data[par1] = input
			pos += 2
		case 4:
			// fmt.Println("Out:", data[par1])
			out := data[par1]
			switch count {
			case 0:
				current.y = out
				count++
				break
			case 1:
				current.x = out
				count++
				break
			case 2:
				if out == 3 {
					board[paddle.x][paddle.y] = 0
					paddle = point{current.x, current.y}
					board[paddle.x][paddle.y] = 3
				} else if out == 4 {
					board[ball.x][ball.y] = 0
					ball = point{current.x, current.y}
					board[ball.x][ball.y] = 4
				} else if current.x == 0 && current.y == -1 {
					score = out
					fmt.Println(out)
				} else {
					board[current.x][current.y] = out
				}
				count = 0
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
		}
	}
	return score
}

func first() {
	data := getProgram()
	tiles := getTiles(data, 0)
	blocks := 0
	for _, v := range tiles {
		if v.id == 2 {
			blocks++
		}
	}
	fmt.Println(blocks)
}

type point struct {
	x, y int
}

func createBoard(tiles []tile) ([][]int, point, point) {
	width, height := 0, 0
	for _, t := range tiles {
		if t.x > width {
			width = t.x
		}
		if t.y > height {
			height = t.y
		}
	}
	fmt.Println(width, height)
	pos := make([][]int, height+1)
	for i := range pos {
		pos[i] = make([]int, width+1)
	}
	var ball point
	var block point
	for _, t := range tiles {
		pos[t.y][t.x] = t.id
		if t.id == 3 {
			block = point{t.y, t.x}
		} else if t.id == 4 {
			ball = point{t.y, t.x}
		}
	}
	return pos, block, ball
}

func printBoard(board [][]int) {
	for _, row := range board {
		for _, p := range row {
			var t string
			switch p {
			case 0:
				t = " "
			case 1:
				t = "|"
			case 2:
				t = "#"
			case 3:
				t = "-"
			case 4:
				t = "o"
			}
			fmt.Print(t)
		}
		fmt.Println()
	}
}

func second() {
	data := getProgram()
	tiles := getTiles(data, 0)
	board, block, ball := createBoard(tiles)
	printBoard(board)
	fmt.Println(block, ball)
	data = getProgram()
	data[0] = 2
	fmt.Println(runGame(data, 0, block, ball, board))

}

func main() {
	first()
	second()
}
