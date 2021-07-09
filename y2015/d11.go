package y2015

import (
	"bytes"
	"fmt"
	"log"
	"math"
)

// Eleven implements the solution to day 11
func Eleven(input string) (out1 string, out2 string) {
	maxI := int(math.Pow(10, float64(len(input))))
	temp1 := []byte(input)
	var solved bool
	// naive way of incrementing until finding solution
	for i := 0; i < maxI; i++ {
		// increment any instance of not allowed letters and zero out following letters
		ind := bytes.IndexAny(temp1, "iol")
		if ind != -1 {
			temp1[ind]++
			for ii := ind + 1; ii < len(temp1); ii++ {
				temp1[ii] = 'a'
			}
		}

		// checks
		var sequential bool
		var doublePair int
		for j := 0; j < 26; j++ {
			if bytes.Contains(temp1, []byte(fmt.Sprintf("%c%c%c", 'a'+j, 'b'+j, 'c'+j))) {
				sequential = true
			}

			if bytes.Contains(temp1, []byte(fmt.Sprintf("%c%c", 'a'+j, 'a'+j))) {
				doublePair++
			}
		}

		// return if conditions are met
		if i != 0 && sequential && doublePair >= 2 {
			solved = true
			break
		}

		// increment
		temp1[len(temp1)-1]++
		prepend := ""
		for k := len(temp1) - 1; k >= 0; k-- {
			if temp1[k] <= 'z' {
				break
			}

			temp1[k] = 'a'
			if k == 0 {
				prepend = "a"
				continue
			}
			temp1[k-1]++
		}
		temp1 = append([]byte(prepend), temp1...)
	}
	if !solved {
		log.Fatal(fmt.Sprintf("No match found in %d iterations, final iteration: %s", maxI, temp1))
	}
	out1 = string(temp1)

	return
}
