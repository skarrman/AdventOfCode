package main

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
)

type Image struct {
	Data []int `json:"input"`
}

const (
	FILE   = "input.json"
	WIDTH  = 25
	HEIGHT = 6
	MAX    = 0x0FFFFFFF
	SIZE   = WIDTH * HEIGHT
)

func getInput() []int {
	dat, err := ioutil.ReadFile(FILE)
	if err != nil {
		panic(err)
	}
	var jsonData Image
	json.Unmarshal(dat, &jsonData)
	return jsonData.Data
}

func getLayers() [][]int {
	image := getInput()
	var layers [][]int
	for i := 0; i < (len(image) / SIZE); i++ {
		lower := i * WIDTH * HEIGHT
		upper := lower + (WIDTH * HEIGHT)
		layers = append(layers, image[lower:upper])
	}
	return layers
}

func first() {
	layers := getLayers()
	minZeros := MAX
	prod := 0
	for _, layer := range layers {
		occ := []int{0, 0, 0}
		for _, pixel := range layer {
			occ[pixel]++
		}
		if minZeros > occ[0] {
			prod = occ[1] * occ[2]
			minZeros = occ[0]
		}
	}
	fmt.Println(prod)
}

func second() {
	layers := getLayers()
	image := make([]int, SIZE)
	for i := len(layers) - 1; i >= 0; i-- {
		layer := layers[i]
		for i, v := range layer {
			if v != 2 {
				image[i] = v
			}
		}
	}
	for i := 0; i < HEIGHT; i++ {
		for j := 0; j < WIDTH; j++ {
			if image[i*WIDTH+j] == 0 {
				fmt.Print(" ")
			} else {
				fmt.Print("X")
			}
		}
		fmt.Println("")
	}
}

func main() {
	first()
	second()
}
