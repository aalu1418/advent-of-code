package y2015

import (
	"fmt"
	"regexp"
	"sort"
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
	re := regexp.MustCompile(`[A-Z][a-z]+`)
	namesAll := re.FindAllString(input, -1)
	sort.Strings(namesAll)
	guests := []string{}
	for _, n := range namesAll {
		if len(guests) != 0 && guests[len(guests)-1] == n {
			continue
		}
		guests = append(guests, n)
	}
	m := make([][]int, len(guests))
	for i := range m {
		m[i] = make([]int, len(guests))
	}
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
		m[g1][g2] += sign * amt
		m[g2][g1] += sign * amt
	}

	for _, r := range m {
		fmt.Println(r)
	}

	return
}
