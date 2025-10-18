package main

import (
	"reflect"

	example "reflection/example"
)

func main() {
	value := reflect.ValueOf(example.FuncList(0))

	for i := range value.NumMethod() {
		method := value.Method(i)
		method.Call(nil)
	}

}
