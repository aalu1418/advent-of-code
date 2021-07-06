package y2015

import (
	"strconv"
	"strings"
)

type variable struct {
	solved   bool
	vars     []string
	numbers  []uint16
	operator string
	value    uint16
}

func checkInt(i string) (str string, num16 uint16) {
	num, err := strconv.Atoi(i)
	num16 = uint16(num)
	if err != nil {
		str = i
	}
	return
}

func converge(gates *map[string]variable) {
	var done int
	for i := 0; i < len(*gates)+10; i++ {
		if done == len(*gates) {
			break
		}
		for i, g := range *gates {
			var skip bool
			skip = g.solved
			for i, v := range g.vars {
				if v != "" {
					if !(*gates)[v].solved {
						skip = true
						continue
					}
					g.numbers[i] = (*gates)[v].value
				}
			}
			if skip {
				continue
			}

			if len(g.vars) == 2 {
				g.value = g.numbers[0] & g.numbers[1]

				if g.operator == "OR" {
					g.value = g.numbers[0] | g.numbers[1]
				}

				if g.operator == "LSHIFT" {
					g.value = g.numbers[0] << g.numbers[1]
				}

				if g.operator == "RSHIFT" {
					g.value = g.numbers[0] >> g.numbers[1]
				}
			}

			if len(g.vars) == 1 {
				g.value = g.numbers[0]

				if g.operator == "NOT" {
					g.value = ^g.numbers[0]
				}
			}

			g.solved = true
			(*gates)[i] = g
			done++
		}
	}
}

// Seven implements the solution for day 7
func Seven(input string) (output uint16, output2 uint16) {
	var gates1 = map[string]variable{}
	var gates2 = map[string]variable{}
	// parsing
	for _, s := range strings.Split(input, "\n") {
		params := strings.Split(s, " ")
		var vars []string
		var nums []uint16
		var op string

		if len(params) == 5 {
			aS, aN := checkInt(params[0])
			bS, bN := checkInt(params[2])
			vars = []string{aS, bS}
			nums = []uint16{aN, bN}
			op = params[1]
		}

		if len(params) == 4 {
			aS, aN := checkInt(params[1])
			vars = []string{aS}
			nums = []uint16{aN}
			op = params[0]
		}

		if len(params) == 3 {
			aS, aN := checkInt(params[0])
			vars = []string{aS}
			nums = []uint16{aN}
		}
		gates1[params[len(params)-1]] = variable{vars: vars, numbers: nums, operator: op}
		gates2[params[len(params)-1]] = variable{vars: vars, numbers: nums, operator: op}
	}

	// convergence
	converge(&gates1)
	gates2["b"] = gates1["a"]
	converge(&gates2)

	output = gates1["a"].value
	output2 = gates2["a"].value
	return
}
