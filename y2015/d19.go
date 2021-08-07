package y2015

import (
	"regexp"
	"sort"
	"strings"
)

// Nineteen implements the solution for day 19
func Nineteen(rawInput string) (out1 int, out2 int) {
	inputs := strings.Split(rawInput, "\n\n")
	pairs := inputs[0]
	input := inputs[1]

	sets := [][]string{}
	shrink := map[string]string{}
	expanded := []string{}
	for _, pair := range strings.Split(pairs, "\n") {
		p := strings.Split(pair, " => ")
		sets = append(sets, p)
		shrink[p[1]] = p[0]
		expanded = append(expanded, p[1])
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

	sort.Strings(expanded)
	sort.Slice(expanded, func(i, j int) bool {
		return len(expanded[i]) > len(expanded[j])
	})

	// pattern recognition solution (quite impressive: https://www.reddit.com/r/adventofcode/comments/3xflz8/day_19_solutions/cy4etju/)
	re := regexp.MustCompile(`[A-Z][a-z]?`)
	parsed := re.FindAllString(input, -1)
	out2 = len(parsed) - 1
	for _, v := range parsed {
		if v == "Rn" || v == "Ar" {
			out2--
		}
		if v == "Y" {
			out2 -= 2
		}
	}

	return
}
