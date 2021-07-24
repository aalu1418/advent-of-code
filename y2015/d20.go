package y2015

import (
	"strconv"
)

func houseGen(num int, max int) []int {
	house := make([]int, num)
	for i := 1; i < num; i++ {
		count := 0
		for j := i; j < num; j += i {
			house[j] += i
			count++
			if count >= max {
				break
			}
		}
	}
	return house
}

// Twenty implements the solution to day 20
func Twenty(input string) (out1 int, out2 int) {
	num, _ := strconv.Atoi(input)
	num1 := num / 10
	num2 := num / 11

	// generate houses
	house := houseGen(num1, num1)
	house2 := houseGen(num2, 50)

	for i := 0; i < len(house); i++ {
		if house[i] > num1 && out1 == 0 {
			out1 = i
		}

		if i < len(house2) && house2[i] > num2 && out2 == 0 {
			out2 = i
		}

		if out1 != 0 && out2 != 0 {
			break
		}
	}

	return
}
