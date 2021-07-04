package y2015

import (
	"strings"
)

// Five implements functions for day 5
func Five(input string) (good int, temp2 string) {
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
			break
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
			break
		}

		contains := false
		for _, pattern := range []string{"ab", "cd", "pq", "xy"} {
			if strings.Contains(s, pattern) {
				contains = true
				break
			}
		}
		if contains == true {
			break
		}

		good++
	}

	return
}
