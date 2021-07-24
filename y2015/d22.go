package y2015

import (
	"fmt"
	"math"
)

type effect struct {
	param *float64
	step  float64
}

type character struct {
	health, attack, armor, mana float64
	effects                     [][]effect
}

func (c *character) call(name string, opponent *character) {
	switch name {
	case "hit":
		if c.attack < 1 { // if attack is 0, don't do anything
			return
		}
		opponent.health -= math.Max(c.attack-opponent.armor, 1)
	case "missile":
		cost := 53.
		if c.mana < cost {
			return
		}
		c.mana -= cost
		opponent.health -= 4

	case "drain":
		cost := 73.
		if c.mana < cost {
			return
		}
		opponent.health -= 2
		c.health += 2

	case "shield":
		cost := 113.
		if c.mana < cost {
			return
		}

	case "poison":
		cost := 173.
		if c.mana < cost {
			return
		}

	case "recharge":
		cost := 229.
		if c.mana < cost {
			return
		}

	}
}

// Twentytwo implements the solution for day 22
func Twentytwo(bossData string) (out1 int, out2 int) {
	boss := character{health: 100, attack: 100, armor: 100, mana: 100}

	fmt.Println(boss)

	return
}
