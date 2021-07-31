package y2015

import (
	"strconv"
	"strings"
)

func createArray(start []int, sum int, next []int, c chan []int, check *bool, maxLen int, target int) {
	for i := 0; i < len(next); i++ {
		nextStart := append(start, next[i])
		if len(nextStart) == maxLen {
			if sum+next[i] == target {
				*check = true
				c <- nextStart
			}
			continue
		}

		// filter out appended value from next
		nextA := []int{}
		for j := 0; j < len(next); j++ {
			if i == j {
				continue
			}
			nextA = append(nextA, next[j])
		}
		createArray(nextStart, sum+next[i], nextA, c, check, maxLen, target)
	}
}

func generateCombs(w []int, target int, c chan []int) {
	defer close(c)
	found := false
	for i := 4; i <= len(w); i++ {
		createArray([]int{}, 0, w, c, &found, i, target)
		if found {
			break
		}
	}
}

// Twentyfour implements the solution to day 24
func Twentyfour(input string) (out1 int, out2 int) {
	weights := []int{}
	total := 0
	for _, w := range strings.Split(input, "\n") {
		weight, _ := strconv.Atoi(w)
		total += weight
		weights = append(weights, weight)
	}

	// pt 1
	ch := make(chan []int)
	go generateCombs(weights, total/3, ch)
	out1 = 100000000000000
	for a := range ch {
		mul := 1
		for _, v := range a {
			mul *= v
		}

		if mul < out1 {
			out1 = mul
		}
	}

	// pt 2
	ch = make(chan []int)
	go generateCombs(weights, total/4, ch)
	out2 = 100000000000000
	for a := range ch {
		mul := 1
		for _, v := range a {
			mul *= v
		}

		if mul < out2 {
			out2 = mul
		}
	}

	return
}
