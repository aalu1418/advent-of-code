package y2015

import (
	"math"
	"regexp"
	"strconv"
)

type effect = []float64

type effectList struct {
	armor, health, mana []float64
}

type character struct {
	health, attack, armor, mana, spent float64
	effects                            effectList
}

func (c *character) call(name string, opponent *character) {
	var cost float64
	switch name {
	case "hit":
		if c.attack < 1 { // if attack is 0, don't do anything
			return
		}
		opponent.health -= math.Max(c.attack-opponent.armor, 1)
	case "missile":
		cost = 53.
		if c.mana < cost {
			return
		}
		opponent.health -= 4

	case "drain":
		cost = 73.
		if c.mana < cost {
			return
		}
		opponent.health -= 2
		c.health += 2

	case "shield":
		cost = 113.
		if c.mana < cost {
			return
		}
		c.effects.armor = []float64{7, 0, 0, 0, 0, 0, -7}

	case "poison":
		cost = 173.
		if c.mana < cost {
			return
		}
		opponent.effects.health = []float64{-3, -3, -3, -3, -3, -3}

	case "recharge":
		cost = 229.
		if c.mana < cost {
			return
		}
		c.effects.mana = []float64{101, 101, 101, 101, 101}
	}

	c.mana -= cost
	c.spent += cost
}

func (c *character) start() {
	if len(c.effects.armor) > 0 {
		c.armor += c.effects.armor[0]
		c.effects.armor = c.effects.armor[1:]
	}
	if len(c.effects.health) > 0 {
		c.health += c.effects.health[0]
		c.effects.health = c.effects.health[1:]
	}
	if len(c.effects.mana) > 0 {
		c.mana += c.effects.mana[0]
		c.effects.mana = c.effects.mana[1:]
	}
}

type game struct {
	you      character
	boss     character
	bossTurn bool
}

func (c *game) endCheck() (end bool, winner string) {
	if c.you.health <= 0 {
		return true, "boss"
	}
	if c.boss.health <= 0 {
		return true, "you"
	}
	return false, ""
}

func simulateGameStep(games []game, minManaSpent *float64, hardMode bool) {
	var nextGames []game
	if len(games) == 0 {
		return
	}

	for _, g := range games {
		// check if end game condition was met at the end of previous move
		check, winner := g.endCheck()
		if check {
			if winner == "you" && g.you.spent < *minManaSpent {
				*minManaSpent = g.you.spent
			}
			continue
		}

		// prune branch if mana spent is already greater
		if g.you.spent > *minManaSpent {
			continue
		}

		if hardMode && !g.bossTurn {
			g.you.health--
			check, _ := g.endCheck() // check to see if you are still alive
			if check {
				continue
			}
		}

		// apply any effects
		g.you.start()
		g.boss.start()

		// check if end game condition was met after effects are applied
		check, winner = g.endCheck()
		if check {
			if winner == "you" && g.you.spent < *minManaSpent {
				*minManaSpent = g.you.spent
			}
			continue
		}

		if g.bossTurn {
			g.boss.call("hit", &g.you)
			g.bossTurn = false
			nextGames = append(nextGames, g)
			continue
		}

		for _, a := range []string{"missile", "drain", "shield", "poison", "recharge"} {
			// can't double up on effects
			if (a == "shield" && len(g.you.effects.armor) != 0) || (a == "recharge" && len(g.you.effects.mana) != 0) || (a == "poison" && len(g.boss.effects.health) != 0) {
				continue
			}

			temp := g
			temp.you.call(a, &temp.boss)
			temp.bossTurn = true
			nextGames = append(nextGames, temp)
		}
	}

	simulateGameStep(nextGames, minManaSpent, hardMode)
}

// Twentytwo implements the solution for day 22
func Twentytwo(bossData string) (out1 float64, out2 float64) {
	boss := character{}
	you := character{health: 50, mana: 500}

	// parse boss stats
	re := regexp.MustCompile(`\d+`)
	numStr := re.FindAllString(bossData, -1)
	stat := []*float64{&boss.health, &boss.attack}
	for i, num := range numStr {
		statFloat, _ := strconv.ParseFloat(num, 64)
		*stat[i] = statFloat
	}

	out1 = 1000000000000.
	out2 = 1000000000000.
	simulateGameStep([]game{game{you, boss, false}}, &out1, false)
	simulateGameStep([]game{game{you, boss, false}}, &out2, true)
	return
}
