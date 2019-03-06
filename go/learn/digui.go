// hello
package main

import (
	"fmt"
)

func fact(n int) int64 {
	if n == 1 {
		return 1
	}
	return int64(n) * fact(n-1)

}

func main() {
	fmt.Println(fact(60))
	fmt.Println("Hello World!")
}