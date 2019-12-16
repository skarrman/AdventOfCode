package main

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
)

var dirs = []int{1, 2, 3, 4}

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

type program struct {
	data          []int
	pos, relative int
}

func runProgram(prog program, input int) (program, int) {
	data := prog.data
	pos := prog.pos
	relative := prog.relative
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
			// fmt.Println("In:", input)
			data[par1] = input
			pos += 2
		case 4:
			out := data[par1]
			// fmt.Println("Out:", out)
			pos += 2
			return program{data, pos, relative}, out
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
	return program{data, -1, -1}, 99
}

func getProgram() []int {
	data := readData("input.json")
	freeData := make([]int, 10000)
	return append(data, freeData...)
}

type point struct {
	x, y int
}

func dif(dir int) point {
	switch dir {
	case 1:
		return point{0, -1}
	case 2:
		return point{0, 1}
	case 3:
		return point{-1, 0}
	case 4:
		return point{1, 0}
	default:
		panic(fmt.Sprintf("%d %s", dir, "not a valid dir"))
	}
}

func intAbs(n int) int {
	if n < 0 {
		return -n
	}
	return n
}

func printMap(m map[point]int, current point) {
	minX, maxX, minY, maxY := 0, 0, 0, 0
	for p := range m {
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
	for y := minY; y <= maxY; y++ {
		for x := minX; x <= maxX; x++ {
			po := point{x, y}
			p, is := m[point{x, y}]
			if y == 0 && x == 0 {
				fmt.Print("0")
			} else if po.x == current.x && po.y == current.y {
				fmt.Print("D")
			} else if p == 0 && is {
				fmt.Print("#")
			} else if p == 1 && is {
				fmt.Print(".")
			} else {
				fmt.Print(" ")
			}
		}
		fmt.Println()
	}
}

type queueElem struct {
	pnt point
	pre *queueElem
}

func backTrace(elm *queueElem) []point {
	revPath := make([]point, 0)
	for elm != nil {
		revPath = append(revPath, (*elm).pnt)
		elm = elm.pre
	}
	return revPath
}

func reverse(revPath []point) []point {
	for i, j := 0, len(revPath)-1; i < j; i, j = i+1, j-1 {
		revPath[i], revPath[j] = revPath[j], revPath[i]
	}
	return revPath
}

func makePath(pointPath []point) []int {
	path := make([]int, 0)
	for i := 1; i < len(pointPath); i++ {
		pre := pointPath[i-1]
		pos := pointPath[i]
		for _, dir := range dirs {
			d := dif(dir)
			p := point{pre.x + d.x, pre.y + d.y}
			if p == pos {
				path = append(path, dir)
			}
		}
	}
	return path
}

func checkPath(path []point, visited map[point]int) {
	for _, p := range path {
		if v, is := visited[p]; !(is && v == 1) {
			panic(fmt.Sprint("checkPath:", p, "not visited"))
		}
	}
}

func pathTo(to, from point, points map[point]int) ([]int, point) {
	points[to] = 1
	queue := []*queueElem{&queueElem{from, nil}}
	visited := make(map[point]bool)
	var curr *queueElem
	for {
		curr = queue[0]
		queue = queue[1:]
		visited[curr.pnt] = true
		if curr.pnt == to {
			break
		}
		for _, dir := range dirs {
			d := dif(dir)
			p := point{curr.pnt.x + d.x, curr.pnt.y + d.y}
			v, _ := points[p]
			if v == 1 && !visited[p] {
				queue = append(queue, &queueElem{p, curr})
			}
		}
	}
	pointPath := reverse(backTrace(curr))
	checkPath(pointPath, points)
	return makePath(pointPath), pointPath[len(pointPath)-2]
}

func goTo(prg program, from, to point, points map[point]int) (program, int, point) {
	if to.x == 0 && to.y == 0 {
		return prg, 1, to
	}
	inputs, sndTolast := pathTo(to, from, points)
	out := 0
	for i := 0; i < len(inputs)-1; i++ {
		prg, out = runProgram(prg, inputs[i])
		if out != 1 && out != 2 {
			panic("Path not clear")
		}
	}
	prg, out = runProgram(prg, inputs[len(inputs)-1])
	return prg, out, sndTolast
}

func first() (map[point]int, point) {
	data := getProgram()
	prg := program{data, 0, 0}
	out := -1
	visited := make(map[point]int)
	current := point{0, 0}
	sndToLast := current
	queue := make([]point, 0)
	queue = append(queue, current)
	goal := current
	for len(queue) != 0 {
		next := queue[0]
		queue = queue[1:]
		if _, v := visited[next]; v {
			continue
		}
		prg, out, sndToLast = goTo(prg, current, next, visited)
		switch out {
		case 1:
			current = next
			visited[current] = 1
		case 0:
			visited[next] = 0
			current = sndToLast
		case 2:
			visited[next] = 2
			path, _ := pathTo(next, point{0, 0}, visited)
			fmt.Println("First:", len(path))
			printMap(visited, next)
			current = next
			goal = next
		default:
			panic(fmt.Sprintln("first: out not expexted:", out))
		}

		for _, dir := range dirs {
			d := dif(dir)
			p := point{current.x + d.x, current.y + d.y}
			if _, hasVisit := visited[p]; !hasVisit {
				queue = append(queue, p)
			}
		}
	}
	printMap(visited, goal)
	return visited, goal
}

type oxFilled struct {
	pnt  point
	time int
}

func second(points map[point]int, start point) {
	queue := make([]oxFilled, 0)
	visited := make(map[point]bool)
	time := make(map[point]int)
	queue = append(queue, oxFilled{start, 0})
	for len(queue) != 0 {
		current := queue[0]
		queue = queue[1:]
		if _, v := visited[current.pnt]; v {
			continue
		}
		visited[current.pnt] = true
		for _, dir := range dirs {
			d := dif(dir)
			p := point{current.pnt.x + d.x, current.pnt.y + d.y}
			v, is := points[p]
			_, hasVisit := visited[p]
			if !hasVisit && is && v == 1 {
				queue = append(queue, oxFilled{p, current.time + 1})
				time[p] = current.time + 1
			}
		}
	}
	max := 0
	for _, v := range time {
		if v > max {
			max = v
		}
	}
	fmt.Println("Second:", max)
}

func main() {
	points, tank := first()
	second(points, tank)
}
