package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

func checkErr(e error) {
	if e != nil {
		panic(e)
	}
}

func read(path string) []string {
	dat, err := ioutil.ReadFile(path)
	checkErr(err)
	return strings.Fields(string(dat))
}

func first() {
	strings := read("input.txt")
	sum := 0
	for _, s := range strings {
		i, err := strconv.Atoi(s)
		checkErr(err)
		sum += (i / 3) - 2
	}
	fmt.Println(sum)
}

func calcFuelNeed(val int) int {
	v := (val / 3) - 2
	if v <= 0 {
		return 0
	}
	return v + calcFuelNeed(v)
}

func second() {
	strings := read("input.txt")
	sum := 0
	for _, s := range strings {
		i, err := strconv.Atoi(s)
		checkErr(err)
		sum += calcFuelNeed(i)
	}
	fmt.Println(sum)
}

func main() {
	first()
	second()
}
