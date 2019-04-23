package main

import "fmt"

var count int64 = 0

func main() {
	re()
}

func re() {
	count++
	fmt.Println(count)
	re()
}
