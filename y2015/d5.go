package y2015

import (
	"strings"
)

// Five implements functions for day 5
func Five(input string) (good int, good2 int) {
	// 1
	for _, s := range strings.Split(input, "\n") {
		vowels := 0
		for _, v := range "aeiou" {
			vowels += strings.Count(s, string(v))
			if vowels >= 3 {
				break
			}
		}
		if vowels < 3 {
			continue
		}

		duplicate := false
		for i := range s {
			if i == len(s)-1 {
				break
			}
			if s[i] == s[i+1] {
				duplicate = true
				break
			}
		}
		if duplicate == false {
			continue
		}

		contains := false
		for _, pattern := range []string{"ab", "cd", "pq", "xy"} {
			if strings.Contains(s, pattern) {
				contains = true
				break
			}
		}
		if contains == true {
			continue
		}

		good++
	}

	// 2
	for _, s := range strings.Split(input, "\n") {
		spaced := false
		contains := false
		for i := range s {
			if i == len(s)-2 {
				break
			}

			if strings.Count(s, s[i:i+2]) == 2 {
				contains = true
			}
			if s[i] == s[i+2] {
				spaced = true
			}

			if spaced && contains {
				good2++
				break
			}
		}
	}

	return
}
