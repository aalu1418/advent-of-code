package y2015

import (
	"fmt"
	"strconv"
	"strings"
)

func indexOf(a *[]string, s string) (ind int) {
	for i, v := range *a {
		if v == s {
			ind = i
			return
		}
	}
	*a = append(*a, s)
	ind = len(*a) - 1
	return
}

// Thirteen implements the solution for day 13
func Thirteen(input string) (out1 int, out2 int) {
	// parsing
	m := [][]int{}
	guests := []string{}
	for _, s := range strings.Split(input, "\n") {
		s = strings.ReplaceAll(s, ".", "")
		sub := strings.Split(s, " ")
		amt, err := strconv.Atoi(sub[3])
		if err != nil {
			fmt.Println("Warning: Failed parsing value")
		}
		sign := 1
		if sub[2] == "lose" {
			sign = -1
		}

		g1 := indexOf(&guests, sub[0])
		g2 := indexOf(&guests, sub[len(sub)-1])
		m[g1][g2] = sign * amt
		fmt.Println(sub[0], g1, sub[len(sub)-1], g2)
	}

	return
}
