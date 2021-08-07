package main

import (
	"fmt"
	"log"
	"os"

	"github.com/aalu1418/advent-of-code/y2015"
)

func main() {
	if len(os.Args) < 3 {
		fmt.Println("Not enough input arguments")
		return
	}

	day := os.Args[1]
	funcArgs := os.Args[2:]
	res, err := y2015.Call(day, funcArgs...)
	if err != nil {
		log.Fatal(err)
	}

	fmt.Println("Outputs:", res[0], res[1])
}
