package y2015

import (
	"math"
	"strconv"
	"strings"
)

type dist map[string]int

var maxInt = int(^uint(0) >> 1)

// Nine implements solution to day 9
func Nine(input string) (minD int, maxD int) {
	// parse
	d := map[string]dist{}
	for _, s := range strings.Split(input, "\n") {
		p := strings.Split(s, " ")
		for _, city := range []string{p[0], p[2]} {
			if d[city] == nil {
				d[city] = dist{}
			}
		}
		d[p[0]][p[2]], _ = strconv.Atoi(p[4])
		d[p[2]][p[0]], _ = strconv.Atoi(p[4])
	}

	minD = maxInt
	maxD = maxInt
	for city := range d {
		distance := step(city, &d, map[string]bool{}, false)
		minD = int(math.Min(float64(minD), float64(distance)))

		distance = step(city, &d, map[string]bool{}, true)
		maxD = int(math.Min(float64(maxD), float64(distance)))
	}
	maxD *= -1

	return
}

func step(city string, paths *map[string]dist, visited map[string]bool, max bool) (dist int) {
	// copy
	visitNext := map[string]bool{}
	for i := range visited {
		visitNext[i] = visited[i]
	}

	factor := 1
	if max {
		factor = -1
	}
	visitNext[city] = true

	var d []int
	for next := range (*paths)[city] {
		if !(visited)[next] {
			d = append(d, factor*(*paths)[city][next]+step(next, paths, visitNext, max))
		}
	}

	if len(d) == 0 {
		return 0
	}

	dist = maxInt
	for _, v := range d {
		if v < dist {
			dist = v
		}
	}

	return
}
