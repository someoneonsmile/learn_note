package main

import "fmt"

func main() {

	// 接口类型变量的定义
	var a Animal
	fmt.Println(a)

	// 实例化结构体, 并赋值给 interface
	a = dog{"joy", 2}
	// a = &dog{name: "joy", age: 2}

	// 通过接口访问方法
	a.Run()
	fmt.Println(a.Name())

	// 接口转回原来类型
	a2 := a.(dog)
	fmt.Println(a2.age)

	// 另外, 类型断言返回值 也可以有第二个 bool 返回值, 表示断言是否成功
	if a3, ok := a.(dog); ok {
		fmt.Println(a3.age)
	}

}

// 定义接口
type Animal interface {
	Run()
	Name() string
}

// 实现接口, 实现接口的不一定只是结构体, 也可以是函数对象
type dog struct {
	name string
	age  int
}

func (dog) Run() {
	fmt.Println("running")
}

// 接收参数 dog 不可以是指针类型, 否则不认为是实现了接口
func (d dog) Name() string {
	return d.name
}
