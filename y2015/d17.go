package y2015

import (
	"sort"
	"strconv"
	"strings"
)

func combs(a []int, max int, num int, count *int, counts *[]int) {
	for i, v := range a {
		if max-v == 0 {
			(*count)++
			*counts = append(*counts, num+1)
			continue
		}

		if max-v < 0 {
			continue
		}
		combs(a[i+1:], max-v, num+1, count, counts)
	}
}

// Seventeen implements the solution to day 17
func Seventeen(input string) (out1 int, out2 int) {
	sizes := []int{}
	for _, s := range strings.Split(input, "\n") {
		v, _ := strconv.Atoi(s)
		sizes = append(sizes, v)
	}

	counts := []int{} //number of containers used
	combs(sizes, 150, 0, &out1, &counts)
	sort.Ints(counts)
	for _, v := range counts {
		if v != counts[0] {
			break
		}
		out2++
	}
	return
}
