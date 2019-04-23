package main

import "fmt"

func main() {

	p := person{"1", 0}
	fmt.Println(p)

	p = person{string: "2", age: 0}
	fmt.Println(p)

	p = person{string: "3"}
	fmt.Println(p)

	// 获取成员, 注意不能用强制类型转换: p.(string)
	fmt.Println(p.string)
}

type person struct {
	string // 匿名成员, 直接写类型
	age    int
}
