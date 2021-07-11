package y2015

import (
	"fmt"
	"regexp"
	"strconv"
	"strings"
)

type ingredient struct {
	c, d, f, t, cal int
}

// Fifteen implements the solution to day 15
func Fifteen(input string) (out1 int, out2 int) {
	// parse
	ing := map[string]ingredient{}
	for _, s := range strings.Split(input, "\n") {
		re := regexp.MustCompile(`-?\d+`)
		v := re.FindAllString(s, -1)
		re = regexp.MustCompile(`[A-Z][a-z]+`)
		k := re.FindString(s)

		vI := []int{}
		for _, vS := range v {
			temp, _ := strconv.Atoi(vS)
			vI = append(vI, temp)
		}
		ing[k] = ingredient{vI[0], vI[1], vI[2], vI[3], vI[4]}
	}

	fmt.Println(ing)

	return
}
