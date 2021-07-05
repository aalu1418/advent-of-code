package y2015

import (
	"math"
	"regexp"
	"strconv"
	"strings"
)

// Six implements the solution for day 6
func Six(input string) (count int, count2 int) {
	var grid [1000][1000]bool
	var grid2 [1000][1000]int

	for _, s := range strings.Split(input, "\n") {
		re := regexp.MustCompile(`[-]?\d[\d]*[\.]?[\d{2}]*`)
		coords := re.FindAllString(s, -1)
		x0, _ := strconv.Atoi(coords[0])
		y0, _ := strconv.Atoi(coords[1])
		x1, _ := strconv.Atoi(coords[2])
		y1, _ := strconv.Atoi(coords[3])

		value := false // default to off
		inc := -1      // default to decrease brightness
		if strings.Contains(s, "turn on") {
			value = true
			inc = 1
		}

		toggle := false
		if strings.Contains(s, "toggle ") {
			toggle = true
			inc = 2
		}

		for i := x0; i <= x1; i++ {
			for j := y0; j <= y1; j++ {
				grid2[i][j] = int(math.Max(float64(grid2[i][j]+inc), 0))

				if toggle {
					grid[i][j] = !grid[i][j]
					continue
				}
				grid[i][j] = value
			}
		}

	}

	for i := 0; i < len(grid); i++ {
		for j := 0; j < len(grid[0]); j++ {
			if grid[i][j] {
				count++
			}
			count2 += grid2[i][j]
		}
	}

	return
}
