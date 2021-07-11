package y2015

import (
	"regexp"
	"strconv"
	"strings"
)

type reindeer struct {
	total  int
	rate   int
	points int

	rest int
	move int

	resting bool
	maxRest int
	maxMove int
}

func (r *reindeer) step() {
	if !r.resting {
		r.total++
		r.move++
	}

	if r.resting {
		r.rest++
	}

	if r.move == r.maxMove || r.rest == r.maxRest {
		r.rest = 0
		r.move = 0
		r.resting = !r.resting
	}
}

// Fourteen implements the solution to day 14
func Fourteen(input string, time string) (out1 int, out2 int) {
	maxT, _ := strconv.Atoi(time)

	// parse
	deer := []reindeer{}
	for _, s := range strings.Split(input, "\n") {
		re := regexp.MustCompile(`\d+`)
		v := re.FindAllString(s, -1)
		rate, _ := strconv.Atoi(v[0])
		maxMove, _ := strconv.Atoi(v[1])
		maxRest, _ := strconv.Atoi(v[2])

		deer = append(deer, reindeer{rate: rate, maxMove: maxMove, maxRest: maxRest})
	}

	// time
	for i := 0; i < maxT; i++ {
		var win []int
		for ii := range deer {
			deer[ii].step()

			total := deer[ii].total * deer[ii].rate
			if total > out1 {
				out1 = total
				win = []int{ii}
				continue
			}
			if out1 == total {
				win = append(win, ii)
			}
		}

		for _, w := range win {
			deer[w].points++
			if deer[w].points > out2 {
				out2 = deer[w].points
			}
		}
	}

	return
}
