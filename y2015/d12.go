package y2015

import (
	"bytes"
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

func filter(in []byte) []byte {
	i := bytes.Index(in, []byte(":\"red\""))

	count := 0
	for ii := i; ii >= 0; ii-- {
		v := in[ii]
		in[ii] = byte('X')

		if v == '}' {
			count++
		}

		if v == '{' && count == 0 {
			break
		}

		if v == '{' {
			count--
		}
	}

	count = 0
	for ii := i; ii < len(in); ii++ {
		v := in[ii]
		in[ii] = byte('X')

		if v == '{' {
			count++
		}

		if v == '}' && count == 0 {
			break
		}

		if v == '}' {
			count--
		}
	}

	return in
}

// Twelve implements the solution to day 12
func Twelve(input string) (out1 int, out2 int) {
	out1 = count(input)

	inB := []byte(input)
	for i := 0; i < len(input); i++ {
		if !bytes.Contains(inB, []byte(":\"red\"")) {
			break
		}
		inB = filter(inB)
	}
	out2 = count(string(inB))

	return
}
