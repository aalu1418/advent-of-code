package y2015

import (
	"regexp"
	"strings"
)

// Nineteen implements the solution for day 19
func Nineteen(pairs string, input string) (out1 int, out2 int) {
	sets := [][]string{}
	for _, pair := range strings.Split(pairs, "\n") {
		sets = append(sets, strings.Split(pair, " => "))
	}

	match := map[string]bool{}
	for _, io := range sets {
		re := regexp.MustCompile(io[0])
		indices := re.FindAllStringIndex(input, -1)
		for _, index := range indices {
			newString := input[:index[0]] + io[1] + input[index[1]:]
			match[newString] = true
		}
	}
	out1 = len(match)
	return
}
