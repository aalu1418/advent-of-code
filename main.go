package main

import (
	"errors"
	"fmt"
	"log"
	"os"
	"reflect"

	"github.com/aalu1418/advent-of-code/y2015"
)

var funcs = map[string]map[string]interface{}{
	"2015": y2015.Interfaces,
}

// Call allows for arg strings to refer to functions
// https://stackoverflow.com/questions/18017979/golang-pointer-to-function-from-string-functions-name
func Call(year string, function string, params ...string) (result []reflect.Value, err error) {
	f := reflect.ValueOf(funcs[year][function])
	if len(params) != f.Type().NumIn() {
		err = errors.New("incorrect parameters")
		return
	}
	in := make([]reflect.Value, len(params))
	for k, param := range params {
		in[k] = reflect.ValueOf(param)
	}
	result = f.Call(in)
	return
}

func main() {
	if len(os.Args) < 3 {
		fmt.Println("Not enough input arguments")
		return
	}

	year := os.Args[1]
	funcName := os.Args[2]
	funcArgs := os.Args[3:]
	res, err := Call(year, funcName, funcArgs...)
	if err != nil {
		log.Fatal(err)
	}

	fmt.Println("Outputs:", res[0], res[1])
}
