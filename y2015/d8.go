package y2015

import (
	"strconv"
	"strings"
)

// Eight implements the solution for day 8
func Eight(input string) (output1 int, output2 int) {
	for _, s := range strings.Split(input, "\n") {
		parsed, _ := strconv.Unquote(s)
		quoted := strconv.Quote(s)
		output1 += len(s) - len(parsed)
		output2 += len(quoted) - len(s)
	}
	return
}
