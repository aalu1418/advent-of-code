package y2015

import (
	"strings"
)

// One implements Day 1
func One(input string) (floor int, index int) {
	// 1
	open := strings.Count(input, "(")
	close := strings.Count(input, ")")
	floor = open - close

	// 2
	count := 0
	for i := 0; i < len(input); i++ {
		if string(input[i]) == "(" {
			count++
		}

		if string(input[i]) == ")" {
			count--
		}

		if count < 0 {
			index = i + 1
			break
		}
	}
	return
}
