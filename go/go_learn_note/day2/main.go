package main

import (
	"fmt"
)

// 继承
func main() {
	v := new(Voodoo)
	v.Magic()
	v.MoreMagic()
}

type Base struct{}

func (Base) Magic() {
	fmt.Println("base magic")
}

func (base Base) MoreMagic() {
	base.Magic()
	base.Magic()
}

type Voodoo struct {
	Base
}

func (Voodoo) Magic() {
	fmt.Println("voodoo magic")
}
