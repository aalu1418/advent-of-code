package y2015

import (
	"strconv"
	"strings"
)

func runProgram(reg map[string]int, cmds []string) {
	i := 0
	for i >= 0 && i < len(cmds) {
		cmd := strings.Split(cmds[i], " ")
		r := string(cmd[1][0])
		step := 1

		switch cmd[0] {
		case "hlf":
			reg[r] /= 2
		case "tpl":
			reg[r] *= 3
		case "inc":
			reg[r]++
		case "jmp":
			step, _ = strconv.Atoi(cmd[1])
		case "jie":
			if reg[r]%2 == 0 {
				step, _ = strconv.Atoi(cmd[2])
			}
		case "jio":
			if reg[r] == 1 {
				step, _ = strconv.Atoi(cmd[2])
			}
		}
		i += step
	}
}

// Twentythree implements the solution to day 23
func Twentythree(input string) (out1 int, out2 int) {
	cmds := strings.Split(input, "\n")
	reg := map[string]int{}
	reg2 := map[string]int{"a": 1}

	runProgram(reg, cmds)
	runProgram(reg2, cmds)

	out1 = reg["b"]
	out2 = reg2["b"]

	return
}
