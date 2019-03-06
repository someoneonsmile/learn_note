package main

import (
	"bytes"
	"fmt"
	"time"
)

var p = fmt.Println

func main() {

	b := time.Nanosecond
	p(b)
	var buffer bytes.Buffer
	for n := 0; n < 1000000000; n++ {
		buffer.WriteString("test")
	}
	e := time.Nanosecond
	print(e - b)

}
