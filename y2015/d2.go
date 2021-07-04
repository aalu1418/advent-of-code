package y2015

import (
	"math"
	"strconv"
	"strings"
)

// Two implements Day 2
func Two(list string) (total int, temp2 string) {
	// 1
	split := strings.Split(list, "\n")
	for _, v := range split {
		dim := strings.Split(v, "x")
		l, _ := strconv.ParseFloat(dim[0], 64)
		w, _ := strconv.ParseFloat(dim[1], 64)
		h, _ := strconv.ParseFloat(dim[2], 64)
		smallest := math.Inf(1)

		for _, a := range [3]float64{l * w, w * h, h * l} {
			total += int(2 * a)
			smallest = math.Min(smallest, a)
		}
		total += int(smallest)
	}

	return
}
