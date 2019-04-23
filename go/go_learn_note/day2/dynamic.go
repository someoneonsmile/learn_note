package main

import "fmt"

func main() {
	dyn(&entry{})
}

type A interface {
	sing()
}

type B interface {
	run()
}

type entry struct{}

func (_ *entry) sing() {
	fmt.Println("sing")
}

func (_ *entry) run() {
	fmt.Println("run")
}

// go 的动态特性
// 接口类型转换, 只要 a 的底层方法定义了 run 方法就行
func dyn(a A) {
	a.(B).run()
}
