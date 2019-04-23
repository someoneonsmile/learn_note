package main

import "fmt"

func main() {

	// 创建对象
	duck := &Duck{age: 8, name: "小灰"}
	fmt.Println(duck)

	// 值对象方法
	d := duck.setAge(10)
	fmt.Println(duck)
	fmt.Println(d)

	// 引用对象方法
	duck.setAge2(20)
	fmt.Println(duck)
	duck2 := Duck{age: 2, name: "小灰2"}
	duck2.setAge2(30)
	fmt.Println(duck2)

	// 赋值给接口
	var v singer = duck

	// 判断接口实际类型
	// 引用类型与值类型为不同的类型
	switch v.(type) {
	case Duck:
		fmt.Println("duck")
	case *Duck:
		fmt.Println("*duck")
	default:
		fmt.Println("default")
	}

	// 接口转为实际类型
	d2 := v.(*Duck)
	fmt.Println(d2)
}

// 定义接口
type singer interface {
	sing()
}

// 定义结构体
type Duck struct {
	Age  int32  `年龄`
	Name string `姓名`
}

// 定义实例方法
func (d Duck) sing() {
	println("sing")
}

func (d Duck) setAge(age int32) Duck {
	d.Age = age
	return d
}

func (d *Duck) setAge2(age int32) *Duck {
	d.Age = age
	return d
}
