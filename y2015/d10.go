package y2015

import (
	"fmt"
	"strings"
)

// Ten implements the solution to day 10
func Ten(input string) (out1 int, out2 int) {
	temp := input
	for i := 0; i < 50; i++ {
		temp = parse(temp)

		if i == 40-1 {
			out1 = len(temp)
		}
	}
	out2 = len(temp)

	return
}

func addSeq(a *[]string, i int, c string) {
	*a = append(*a, fmt.Sprintf("%d", i))
	*a = append(*a, c)
}

func parse(in string) (out string) {
	outArray := []string{}
	count := 0
	char := ""
	for i, c := range in {
		cS := string(c)
		if cS != char {
			if i != 0 {
				addSeq(&outArray, count, char)
			}
			count = 1
			char = cS
			continue
		}
		count++
	}
	addSeq(&outArray, count, char) // add final character
	out = strings.Join(outArray, "")
	return
}
