package main

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
)

type OrbData struct {
	Ce  string `json:"ce"`
	Orb string `json:"orb"`
}

type Data struct {
	Data []OrbData `json:"input"`
}

func getData(num int) map[string][]string {
	dat, err := ioutil.ReadFile("input.json")
	if err != nil {
		panic(err)
	}
	var jsonData Data
	json.Unmarshal(dat, &jsonData)
	lst := jsonData.Data
	orbs := make(map[string][]string)
	for _, o := range lst {
		orbs[o.Ce] = append(orbs[o.Ce], o.Orb)
		if num == 2 {
			orbs[o.Orb] = append(orbs[o.Orb], o.Ce)
		}

	}
	return orbs
}

func getPlanets(orbs map[string][]string) map[string]int {
	planets := make(map[string]int)
	for k, vs := range orbs {
		planets[k] = 0
		for _, v := range vs {
			planets[v] = 0
		}
	}
	return planets
}

func calcOrbit(step int, planet string, planets map[string]int, orbs map[string][]string) {
	planets[planet] = step
	for _, p := range orbs[planet] {
		calcOrbit(step+1, p, planets, orbs)
	}
}

func search(curr string, goal string, steps int, visited map[string]bool, orbs map[string][]string) bool {
	if visited[curr] == true {
		return false
	}
	visited[curr] = true
	if curr == goal {
		fmt.Println("Second", steps-1)
		return true
	}
	found := false
	for _, p := range orbs[curr] {
		f := search(p, goal, steps+1, visited, orbs)
		found = found || f
	}
	return found
}
func first(orbs map[string][]string) {
	planets := getPlanets(orbs)
	calcOrbit(0, "COM", planets, orbs)
	sum := 0
	for _, v := range planets {
		sum += v
	}
	fmt.Println("First:", sum)
}

func second(orbs map[string][]string) {
	search("YOU", "SAN", -1, make(map[string]bool), orbs)
}

func main() {
	orbs := getData(1)
	first(orbs)
	orbs = getData(2)
	second(orbs)
}
