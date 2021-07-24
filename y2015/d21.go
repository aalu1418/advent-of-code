package y2015

import (
	"math"
	"regexp"
	"strconv"
	"strings"
)

type stats = [3]float64

// Twentyone implements the solution to day 21
func Twentyone(shop string, bossStats string) (out1 float64, out2 float64) {
	// parsing
	you := stats{100, 0, 0}
	boss := stats{}
	re := regexp.MustCompile(`\d+`)
	for i, v := range re.FindAllString(bossStats, -1) {
		f, _ := strconv.ParseFloat(v, 64)
		boss[i] = f
	}
	store := [][]stats{}
	for indClass, i := range strings.Split(shop, "\n\n") {
		class := []stats{}

		if indClass != 0 { // empty armor and ring slot
			class = append(class, stats{})
		}

		if indClass == 2 { // add second empty ring slot
			class = append(class, stats{})
		}

		for ind, j := range strings.Split(i, "\n") {
			if ind == 0 {
				continue
			}
			item := stats{}

			for ii, v := range re.FindAllString(j, -1) {
				if indClass == 2 && ii == 0 {
					continue
				}
				f, _ := strconv.ParseFloat(v, 64)
				if indClass == 2 {
					item[ii-1] = f
					continue
				}
				item[ii] = f
			}
			class = append(class, item)
		}
		store = append(store, class)
	}

	survive := func(d, a float64) bool {
		rBoss := math.Ceil(boss[0] / math.Max(d-boss[2], 1))
		rYou := math.Ceil(you[0] / math.Max(boss[1]-a, 1))

		return rYou >= rBoss
	}

	out1 = 10000000
	for _, weapon := range store[0] { // loop weapons
		for _, armor := range store[1] { // loop armor
			for ii := 0; ii < len(store[2]); ii++ { // loop rings
				for jj := ii; jj < len(store[2]); jj++ { // loop remaining rings
					ring0 := store[2][ii]
					ring1 := store[2][jj]

					cost := weapon[0] + armor[0] + ring0[0] + ring1[0]
					dStat := weapon[1] + armor[1] + ring0[1] + ring1[1]
					aStat := weapon[2] + armor[2] + ring0[2] + ring1[2]

					if cost < out1 && survive(dStat, aStat) {
						out1 = cost
					}

					if cost > out2 && !survive(dStat, aStat) {
						out2 = cost
					}
				}
			}
		}
	}
	return
}
