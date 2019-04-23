package main

type Foo map[string]string
type Bar struct {
	thingOne string
	thingTwo int
}

// make, new 的使用不能用反
// slice, map, channel 的创建使用 make
// 结构体的创建使用 new(Struct) / &Struct{}
func main() {
	// OK
	y := new(Bar)
	(*y).thingOne = "hello"
	(*y).thingTwo = 1

	// NOT OK
	//z := make(Bar) // 编译错误：cannot make type Bar
	//(*y).thingOne = "hello"
	//(*y).thingTwo = 1

	// OK
	x := make(Foo)
	x["x"] = "goodbye"
	x["y"] = "world"

	// NOT OK
	u := new(Foo)
	(*u)["x"] = "goodbye" // 运行时错误!! panic: assignment to entry in nil map
	(*u)["y"] = "world"
}
