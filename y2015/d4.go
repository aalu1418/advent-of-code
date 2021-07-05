package y2015

import (
	"crypto/md5"
	"fmt"
	"io"
)

// Four implements the logic for the 4th day
func Four(input string) (number int, number2 int) {
	for i := 1; i < 1000000000; i++ {
		if i%500000 == 0 {
			fmt.Printf("Reached %d checks\n", i)
		}
		h := md5.New()
		io.WriteString(h, fmt.Sprintf("%s%d", input, i))
		hash := fmt.Sprintf("%x", h.Sum(nil))
		if number == 0 && hash[:5] == "00000" {
			number = i
			fmt.Println("'00000' match found")
		}

		if number2 == 0 && hash[:6] == "000000" {
			number2 = i
			fmt.Println("'000000' match found")
		}

		if number != 0 && number2 != 0 {
			break
		}

	}
	if number == 0 {
		fmt.Println("Warn: No hash found")
	}

	return
}
