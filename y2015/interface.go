package y2015

import (
	"errors"
	"reflect"
)

// Interfaces exports string mapping relation to functions
var funcs = map[string]interface{}{
	"1":  One,
	"2":  Two,
	"3":  Three,
	"4":  Four,
	"5":  Five,
	"6":  Six,
	"7":  Seven,
	"8":  Eight,
	"9":  Nine,
	"10": Ten,
	"11": Eleven,
	"12": Twelve,
	"13": Thirteen,
	"14": Fourteen,
	"15": Fifteen,
	"16": Sixteen,
	"17": Seventeen,
	"18": Eighteen,
	"19": Nineteen,
	"20": Twenty,
	"21": Twentyone,
	"22": Twentytwo,
	"23": Twentythree,
	"24": Twentyfour,
	"25": Twentyfive,
}

// Call allows for arg strings to refer to functions
// https://stackoverflow.com/questions/18017979/golang-pointer-to-function-from-string-functions-name
func Call(day string, params ...string) (result []reflect.Value, err error) {
	f := reflect.ValueOf(funcs[day])
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
