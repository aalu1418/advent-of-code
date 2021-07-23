package y2015

import (
	"strings"
)

// Grid implements the x,y grid for the problem
type Grid = [][]byte

func checkSurrounding(x int, y int, table *Grid, stuck bool) byte {
	maxRow := len(*table)
	maxCol := len((*table)[0])
	steps := []int{-1, 0, 1}

	var count int
	for _, dx := range steps {
		for _, dy := range steps {
			if (dx == 0 && dy == 0) || (x+dx < 0) || (y+dy < 0) || (x+dx >= maxRow) || (y+dy >= maxCol) {
				continue
			}

			if (*table)[x+dx][y+dy] == '#' {
				count++
			}
		}
	}

	if stuck && (x == maxRow-1 || x == 0) && (y == maxCol-1 || y == 0) {
		return '#'
	}

	if ((*table)[x][y] == '#' && (count == 2 || count == 3)) || ((*table)[x][y] == '.' && count == 3) {
		return '#'
	}

	return '.'
}

// Eighteen implements the solution for day 18
func Eighteen(input string) (out1 int, out2 int) {
	rounds := 100

	grid0 := Grid{}
	// parse
	for _, v := range strings.Split(input, "\n") {
		grid0 = append(grid0, []byte(v))
	}

	loop := func(gridInput Grid, broken bool, v *int) {
		grid := gridInput
		for r := 0; r < rounds; r++ {
			newGrid0 := Grid{}
			for i := 0; i < len(grid); i++ {
				newRow0 := []byte{}
				for j := 0; j < len(grid[0]); j++ {
					light := checkSurrounding(i, j, &grid, broken)
					newRow0 = append(newRow0, light)
					if r == rounds-1 && light == '#' {
						*v++
					}
				}
				newGrid0 = append(newGrid0, newRow0)
			}
			grid = newGrid0
		}
	}

	loop(grid0, false, &out1)

	grid1 := grid0
	grid1[0][0] = '#'
	grid1[0][len(grid1[0])-1] = '#'
	grid1[len(grid1[0])-1][0] = '#'
	grid1[len(grid1[0])-1][len(grid1[0])-1] = '#'

	loop(grid1, true, &out2)

	return
}
