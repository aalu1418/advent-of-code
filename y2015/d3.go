package y2015

type y map[int]bool
type visit map[int]y

type santa struct {
	X, Y    int
	Visited visit
}

func (s *santa) move(v string) (newSpace bool) {
	if string(v) == "v" {
		s.Y--
	}

	if string(v) == "^" {
		s.Y++
	}

	if string(v) == ">" {
		s.X++
	}

	if string(v) == "<" {
		s.X--
	}

	if s.Visited[s.X] == nil {
		s.Visited[s.X] = y{}
	}

	if s.Visited[s.X][s.Y] == false {
		newSpace = true
	}

	s.Visited[s.X][s.Y] = true

	return
}

// Three implements Day 3
func Three(input string) (total int, total2 int) {
	sharedMap := visit{0: y{0: true}}
	S1 := santa{Visited: visit{0: y{0: true}}}
	S2 := santa{Visited: sharedMap}
	R1 := santa{Visited: sharedMap}
	total++
	total2++
	for i, v := range input {
		if S1.move(string(v)) {
			total++
		}

		if i%2 == 0 {
			if S2.move(string(v)) {
				total2++
			}
		}

		if i%2 == 1 {
			if R1.move(string(v)) {
				total2++
			}
		}
	}
	return
}
