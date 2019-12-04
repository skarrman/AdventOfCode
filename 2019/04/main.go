package main

import "fmt"

func isValid(n int, second bool) bool {
	last := 0xFFFFFFFF
	dub := false
	len := 0
	occ := []int{0, 0, 0, 0, 0, 0, 0, 0, 0, 0}

	for n > 0 {
		d := n % 10
		occ[d]++
		if !second && last == d {
			dub = true
		}
		if d > last {
			return false
		}
		last = d
		n = n / 10
		len++
	}
	for _, o := range occ {
		if second && o == 2 {
			dub = true
		}
	}
	return dub && len == 6
}

func first() {
	valid := 0
	for i := 123257; i < 647015; i++ {
		if isValid(i, false) {
			valid++
		}
	}
	fmt.Println("First", valid)
}
func second() {
	valid := 0
	for i := 123257; i < 647015; i++ {
		if isValid(i, true) {
			valid++
		}
	}
	fmt.Println("Second:", valid)
}

func main() {
	first()
	second()
}
