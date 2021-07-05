package y2015

import (
	"sort"
	"strconv"
	"strings"
)

// Two implements Day 2
func Two(list string) (total int, length int) {
	split := strings.Split(list, "\n")
	for _, v := range split {
		dim := strings.Split(v, "x")
		l, _ := strconv.ParseFloat(dim[0], 64)
		w, _ := strconv.ParseFloat(dim[1], 64)
		h, _ := strconv.ParseFloat(dim[2], 64)
		d := []float64{l, w, h}
		total += int(2 * (l*w + w*h + h*l))
		sort.Float64s(d)
		total += int(d[0] * d[1])
		length += int(2*(d[0]+d[1]) + (l * w * h))
	}

	return
}
