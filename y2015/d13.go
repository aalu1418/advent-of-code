package y2015

import (
	"fmt"
	"strconv"
	"strings"
)

func contains(a *[]string, s string) bool {
	for _, v := range *a {
		if v == s {
			return true
		}
	}
	return false
}

type intTable map[string]map[string]int

func mapData(x string, y string, v int, table *intTable) {
	if (*table)[x] == nil {
		(*table)[x] = map[string]int{}
	}

	if (*table)[y] == nil {
		(*table)[y] = map[string]int{}
	}

	(*table)[x][y] += v
	(*table)[y][x] += v
}

func combine(m *int, t *intTable, cVal int, visited []string) {
	var v int
	for name := range *t {
		v = cVal
		if !contains(&visited, name) {
			v += (*t)[visited[len(visited)-1]][name]
			combine(m, t, v, append(visited, name))
		}
	}

	if len(visited) == len(*t) {
		v += (*t)[visited[len(visited)-1]][visited[0]]
		if v > *m {
			*m = v
		}
	}
}

// Thirteen implements the solution for day 13
func Thirteen(input string) (out1 int, out2 int) {
	// parsing
	hIndex := intTable{}
	vStart := ""
	for i, s := range strings.Split(input, "\n") {
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
		mapData(sub[0], sub[len(sub)-1], sign*amt, &hIndex)

		// establish starting point
		if i == 0 {
			vStart = sub[0]
		}
	}
	combine(&out1, &hIndex, 0, []string{vStart})

	hIndex["self"] = map[string]int{}
	for i := range hIndex {
		hIndex[i]["self"] = 0
	}

	combine(&out2, &hIndex, 0, []string{vStart})

	return
}
