package main

import "fmt"

func main() {

	var my handler = func(name string) int {
		return len(name)
	}

	// 函数调用
	fmt.Println(my("abc"))
	// 函数调用函数的方法
	fmt.Println(my.add("abc"))
	// 函数转为另一个函数并调用函数的方法
	fmt.Println(handler(doubler).add("abc"))

	// 函数实现了 adder 接口
	process(my)
	// 函数转为另一实现了 adder 接口的函数
	process(handler(doubler))
	// int 转为 myint 类型, myint 类型实现了 adder 接口
	process(myint(8))

	// 别名类型的方法调用
	fmt.Println(IntVector{1, 2, 3}.Sum())

	//: 注意方法也是函数, 不能重载, 即不能有同名函数, 但不同接收者的方法可以同名
}

// 定义一个函数类型 handler
type handler func(name string) int

// 针对这个函数类型可以再定义方法
func (h handler) add(name string) int {
	return h(name) + 10
}

// 定义接口类型
type adder interface {
	add(string) int
}

// 函数参数类型接受实现了 adder 接口的对象 (函数或结构体)
func process(a adder) {
	fmt.Println("process", a.add("a"))
}

func doubler(name string) int {
	return len(name) * 2
}

// 基于一个类型创建一个新类型
type myint int

// 定义别名
// type myint = int

// 为新类型添加方法
func (i myint) add(name string) int {
	return len(name) + int(i)
}

type IntVector []int

// 定义函数的返回值, 并分配空间
func (v IntVector) Sum() (s int) {
	for _, x := range v {
		s += x
	}
	return
}
