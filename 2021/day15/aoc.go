package main

import (
	"fmt"
	"io/ioutil"
	"os"
	"sort"
	"strconv"
	"strings"
)

type pos struct {
	x int
	y int
}

type node struct {
	p      pos
	risk   int
	path   int
	parent pos
}

func parseInput(input string) [][]node {
	var nodes [][]node
	for y, line := range strings.Split(strings.TrimSpace(input), "\n") {
		var nodeRow []node
		for x, char := range strings.Split(line, "") {
			i, _ := strconv.Atoi(char)
			nodeRow = append(nodeRow, node{p: pos{x: x, y: y}, risk: i})
		}
		nodes = append(nodes, nodeRow)
	}
	return nodes
}

func getNode(nodes [][]node, x, y, dim int) (bool, node) {
	distX := 0
	distY := 0
	mappedX := x
	mappedY := y
	if x < 0 || y < 0 || y >= len(nodes)*dim || x >= len(nodes[y%len(nodes)])*dim {
		return false, node{}
	}
	if y >= len(nodes) {
		mappedY = y % len(nodes)
		distY = y / len(nodes)
	}
	if x >= len(nodes[mappedY]) {
		mappedX = x % len(nodes[mappedY])
		distX = x / len(nodes[mappedY])
	}
	risk := (nodes[mappedY][mappedX].risk + distY + distX)
	if risk > 9 {
		res := risk % 10
		risk = res + 1
	}
	return true, node{p: pos{x: x, y: y}, risk: risk}
}

func main() {
	inputBytes, err := ioutil.ReadFile("input.txt")
	if err != nil {
		panic("couldn't read input")
	}
	dim := 1
	if os.Getenv("part") == "part2" {
		dim = 5
	}
	nodes := parseInput(string(inputBytes))
	queue := []node{nodes[0][0]}
	visited := make(map[pos]bool)
	for {
		cur := queue[0]
		queue = queue[1:]
		if visited[cur.p] {
			continue
		}
		if cur.p.y == len(nodes)*dim-1 && cur.p.x == len(nodes[cur.p.y%len(nodes)])*dim-1 {
			fmt.Println(cur.path)
			break
		}
		visited[cur.p] = true
		val, n := getNode(nodes, cur.p.x, cur.p.y-1, dim)
		if val && !visited[n.p] {
			queue = append(queue, node{p: n.p, risk: n.risk, path: cur.path + n.risk, parent: cur.p})
		}
		val, n = getNode(nodes, cur.p.x, cur.p.y+1, dim)
		if val && !visited[n.p] {
			queue = append(queue, node{p: n.p, risk: n.risk, path: cur.path + n.risk, parent: cur.p})
		}
		val, n = getNode(nodes, cur.p.x-1, cur.p.y, dim)
		if val && !visited[n.p] {
			queue = append(queue, node{p: n.p, risk: n.risk, path: cur.path + n.risk, parent: cur.p})
		}
		val, n = getNode(nodes, cur.p.x+1, cur.p.y, dim)
		if val && !visited[n.p] {
			queue = append(queue, node{p: n.p, risk: n.risk, path: cur.path + n.risk, parent: cur.p})
		}
		sort.Slice(queue, func(i, j int) bool {
			return queue[i].path < queue[j].path
		})
	}
}
