package y2015

import (
	"regexp"
	"strconv"
	"strings"
)

var tape = `children: 3
cats: 7
samoyeds: 2
pomeranians: 3
akitas: 0
vizslas: 0
goldfish: 5
trees: 3
cars: 2
perfumes: 1`

// Sixteen implements the solution for day 16
func Sixteen(list string) (out1 string, out2 string) {
	constants := map[string]int{}
	for _, s := range strings.Split(tape, "\n") {
		temp := strings.Split(s, ": ")
		constants[temp[0]], _ = strconv.Atoi(temp[1])
	}

	for _, s := range strings.Split(list, "\n") {
		re := regexp.MustCompile(`\w+: \d+`)
		params := re.FindAllString(s, -1)

		check1 := false
		check2 := false
		for _, p := range params {
			pA := strings.Split(p, ": ")
			v, _ := strconv.Atoi(pA[1])

			if constants[pA[0]] != v {
				check1 = true
			}

			if pA[0] == "cats" || pA[0] == "trees" {
				if !(constants[pA[0]] < v) {
					check2 = true
				}
				continue
			}

			if pA[0] == "pomeranians" || pA[0] == "goldfish" {
				if !(constants[pA[0]] > v) {
					check2 = true
				}
				continue
			}

			if constants[pA[0]] != v {
				check2 = true
			}

		}

		reNum := regexp.MustCompile(`\d+:`)
		indA := reNum.FindAllString(s, 1)
		ind := strings.ReplaceAll(indA[0], ":", "")

		if !check1 {
			out1 = ind
		}

		if !check2 {
			out2 = ind
		}
	}
	return
}
