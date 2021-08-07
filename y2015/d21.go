package y2015

import (
	"math"
	"regexp"
	"strconv"
	"strings"
)

type stats = [3]float64

var shop = `Weapons:    Cost  Damage  Armor
Dagger        8     4       0
Shortsword   10     5       0
Warhammer    25     6       0
Longsword    40     7       0
Greataxe     74     8       0

Armor:      Cost  Damage  Armor
Leather      13     0       1
Chainmail    31     0       2
Splintmail   53     0       3
Bandedmail   75     0       4
Platemail   102     0       5

Rings:      Cost  Damage  Armor
Damage +1    25     1       0
Damage +2    50     2       0
Damage +3   100     3       0
Defense +1   20     0       1
Defense +2   40     0       2
Defense +3   80     0       3`

// Twentyone implements the solution to day 21
func Twentyone(bossStats string) (out1 float64, out2 float64) {
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
