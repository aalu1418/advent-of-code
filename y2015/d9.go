package y2015

import (
	"fmt"
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
	maxD = 0
	for city := range d {
		v := map[string]bool{}
		v[city] = true
		distance := step(city, minD, &d, &v)
		minD = int(math.Min(float64(minD), float64(distance)))

		vMax := map[string]bool{}
		vMax[city] = true
		distance = stepMax(city, &d, vMax)
		fmt.Println(distance)
		maxD = int(math.Max(float64(maxD), float64(distance)))
		break
	}

	return
}

func step(city string, stop int, paths *map[string]dist, visited *map[string]bool) (dist int) {
	minD := maxInt
	minCity := ""

	for next := range (*paths)[city] {
		if !(*visited)[next] && (*paths)[city][next] < minD {
			minD = (*paths)[city][next]
			minCity = next
		}
	}

	(*visited)[minCity] = true
	if len(*visited) == len(*paths) || stop-minD < 0 || minCity == "" {
		return minD
	}
	return minD + step(minCity, stop-minD, paths, visited)
}

// not working
func stepMax(city string, paths *map[string]dist, visited map[string]bool) (dist int) {

	var visitCopy = map[string]bool{}
	for i := range visited {
		visitCopy[i] = visited[i]
	}

	maxD := 0
	for next := range (*paths)[city] {
		if !(visited)[next] {
			(visited)[next] = true
			newD := (*paths)[city][next] + stepMax(next, paths, visitCopy)
			fmt.Println(newD)
			if newD > maxD {
				maxD = newD
			}
		}
	}
	return maxD
}
