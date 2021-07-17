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

func (i *ingredient) Max() {
	if i.c < 0 {
		i.c = 0
	}

	if i.d < 0 {
		i.d = 0
	}

	if i.f < 0 {
		i.f = 0
	}

	if i.t < 0 {
		i.t = 0
	}
}

func objFunc(coords map[string]int, consts *map[string]ingredient) int {
	vals := ingredient{}
	for k, v := range *consts {
		vals.c += coords[k] * v.c
		vals.d += coords[k] * v.d
		vals.f += coords[k] * v.f
		vals.t += coords[k] * v.t
	}
	vals.Max()
	return vals.c * vals.d * vals.f * vals.t
}

// Fifteen implements the solution to day 15
func Fifteen(input string) (out1 int, out2 int) {
	// parse
	ing := map[string]ingredient{}
	coords := map[string]int{}
	total := 100
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
		coords[k] = 1
		total--
	}

	for i := 0; i < total; i++ {
		max := 0
		maxCoords := map[string]int{}
		for k := range coords {
			inputs := map[string]int{}
			for key, val := range coords {
				inputs[key] = val
			}
			inputs[k]++

			v := objFunc(inputs, &ing)
			if v > max {
				max = v
				maxCoords = inputs
			}
		}
		coords = maxCoords
	}
	fmt.Println(coords)
	out1 = objFunc(coords, &ing)

	return
}
