package y2015

import (
	"regexp"
	"strconv"
)

// Twentyfive implements the solution for day 25
func Twentyfive(input string) (out1 int, out2 int) {
	// parse
	re := regexp.MustCompile(`\d+`)
	vals := re.FindAllString(input, -1)
	row, _ := strconv.Atoi(vals[0])
	col, _ := strconv.Atoi(vals[1])

	n := 1 - row + (row+col-1)*(row+col)/2

	out1 = 20151125
	for i := 1; i < n; i++ {
		out1 = (out1 * 252533) % 33554393
	}

	return
}
