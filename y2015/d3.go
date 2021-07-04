package y2015

// Three implements Day 3
func Three(input string) (total int, temp2 string) {
	// 1
	var initY = map[int]bool{
		0: true,
	}
	var visited = map[int]map[int]bool{
		0: initY,
	}
	x := 0
	y := 0
	for _, v := range input {
		if string(v) == "v" {
			y--
		}

		if string(v) == "^" {
			y++
		}

		if string(v) == ">" {
			x++
		}

		if string(v) == "<" {
			x--
		}

		if visited[x][y] == false {
			total++
		}

		if visited[x] == nil {
			visited[x] = map[int]bool{}
		}
		visited[x][y] = true

	}

	return
}
