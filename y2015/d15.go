package y2015

import (
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

func objFunc(coords []int, consts *[]ingredient) (score int, cal int) {
	vals := ingredient{}
	for i := 0; i < len(coords); i++ {
		vals.c += coords[i] * (*consts)[i].c
		vals.d += coords[i] * (*consts)[i].d
		vals.f += coords[i] * (*consts)[i].f
		vals.t += coords[i] * (*consts)[i].t
		vals.cal += coords[i] * (*consts)[i].cal
	}
	vals.Max()
	score = vals.c * vals.d * vals.f * vals.t
	cal = vals.cal
	return
}

func sum(a []int) (t int) {
	for i := 0; i < len(a); i++ {
		t += a[i]
	}
	return
}

// Fifteen implements the solution to day 15
func Fifteen(input string) (out1 int, out2 int) {
	max := 100

	// parse
	ing := []ingredient{}
	for _, s := range strings.Split(input, "\n") {
		re := regexp.MustCompile(`-?\d+`)
		v := re.FindAllString(s, -1)

		vI := []int{}
		for _, vS := range v {
			temp, _ := strconv.Atoi(vS)
			vI = append(vI, temp)
		}
		ing = append(ing, ingredient{vI[0], vI[1], vI[2], vI[3], vI[4]})
	}

	// naive solution (would be interesting to use an optimization package like gonum)

	// create possible combinations
	combs := [][]int{}
	for i := 0; i < len(ing); i++ {
		temp := [][]int{} //temp

		for l := 0; l <= max; l++ {
			if i == 0 {
				a := make([]int, len(ing))
				a[i] = l
				temp = append(temp, a)
			}

			for j := 0; j < len(combs); j++ {
				if i == len(ing)-1 {
					combs[j][i] = max - sum(combs[j])
					continue
				}

				t := make([]int, len(ing))
				copy(t, combs[j])
				t[i] = l
				if sum(t) <= max {
					temp = append(temp, t)
				}
			}
		}

		if i != len(ing)-1 {
			combs = temp //overwrite
		}
	}

	for _, coords := range combs {
		points, cals := objFunc(coords, &ing)
		if points > out1 {
			out1 = points
		}

		if cals == 500 && points > out2 {
			out2 = points
		}
	}

	return
}
