package main

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"strconv"
)

type paths struct {
	Paths [][]string `json:"data"`
}

type event struct {
	dir string
	len int
}

type point struct {
	x int
	y int
}

func readPaths() [][]string {
	dat, e := ioutil.ReadFile("input.json")
	if e != nil {
		panic(e)
	}
	var jsonData paths
	json.Unmarshal(dat, &jsonData)
	return jsonData.Paths
}

func stringToEvent(eventString string) event {
	d := eventString[:1]
	sl := eventString[1:]
	l, e := strconv.Atoi(sl)
	if e != nil {
		panic(e)
	}
	return event{d, l}
}

func getEvents() [][]event {
	input := readPaths()
	events := make([][]event, len(input))
	for i, inp := range input {
		for _, in := range inp {
			ev := stringToEvent(in)
			events[i] = append(events[i], ev)
		}
	}
	return events
}

func createPath(events []event) map[point]int {
	points := make(map[point]int)
	currPoint := point{0, 0}
	currStep := 1
	for _, event := range events {
		for event.len > 0 {
			switch event.dir {
			case "U":
				currPoint = point{currPoint.x, currPoint.y + 1}
			case "D":
				currPoint = point{currPoint.x, currPoint.y - 1}
			case "R":
				currPoint = point{currPoint.x + 1, currPoint.y}
			case "L":
				currPoint = point{currPoint.x - 1, currPoint.y}
			}
			points[currPoint] = currStep
			currStep++
			event.len--
		}
	}
	return points
}

func absInt(n int) int {
	if n < 0 {
		return -n
	}
	return n
}

func solve() {
	events := getEvents()
	fstPoints := createPath(events[0])
	scdPoints := createPath(events[1])
	minDist := 0xFFFFFFFF
	minSteps := 0xFFFFFFFF
	for p, s1 := range scdPoints {
		if s2, found := fstPoints[p]; found {
			dist := absInt(p.x) + absInt(p.y)
			if dist < minDist {
				minDist = dist
			}
			steps := s1 + s2
			if steps < minSteps {
				minSteps = steps
			}
		}
	}
	fmt.Println("First:", minDist)
	fmt.Println("Second:", minSteps)
}

func main() {
	solve()
}
