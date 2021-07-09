package y2015

import (
	"log"
	"regexp"
	"strconv"
)

func count(i string) (count int) {
	re := regexp.MustCompile(`-?\d+`)
	sV := re.FindAllString(i, -1)
	for _, v := range sV {
		vI, err := strconv.Atoi(v)
		if err != nil {
			log.Fatal(err)
		}
		count += vI
	}
	return
}

// Twelve implements the solution to day 12
func Twelve(input string) (out1 int, out2 int) {
	out1 = count(input)

	return
}
