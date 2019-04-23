package main

import (
	"fmt"
	"log"
)

func main() {
	s := str("123")
	s.chc(2, 'c')
	fmt.Println(s)
	defaultP(func(a interface{}) { fmt.Println(a) }, "1")
}

type str string

func (s *str) chc(index int, b byte) {
	cs := []byte(*s)
	if -1 < index || index < len(cs) {
		cs[index] = b
	}
	*s = str(cs)
}

func defaultP(f interface{}, a ...interface{}) {
	switch f.(type) {
	case func():
		f.(func())()
	case func(interface{}):
		f.(func(interface{}))(a[0])
	case func(...interface{}):
		f.(func(...interface{}))(a)
	default:
		log.Println("not a func")
	}
}
