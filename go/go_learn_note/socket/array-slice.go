package main

import "fmt"

func main() {
	var arrayLazy = []string{1: "1", 2: "2"}
	fmt.Println(len(arrayLazy))
	fo(0)
}

func fo(i int) {
	n := 200
	s := make([]int, n)
	for i := 0; i < n*1000; i++ {
		s = append(s, -1)
		if i < 2 {
			s[i] = 1
		} else {
			s[i] = s[i-1] + s[i-2]
		}
		fmt.Println(s[i])
	}
}
