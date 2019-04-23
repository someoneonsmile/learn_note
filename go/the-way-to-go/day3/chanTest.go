package main

import (
	"fmt"
)

func main() {
	ch1 := make(chan int)
	ch2 := make(chan func())

	//go setData(ch1)
	//
	//go getData(ch1, ch2)
	//
	//f := <- ch2
	//f()

	go func() {
		ch1 <- 1
		fmt.Println(1)
	}()

	go func() {
		ch2 <- func() {
			fmt.Println("ch2 func")
		}
	}()

	go func() {
		i := <-ch1
		fmt.Println(i)
	}()

	go func() {
		f := <-ch2
		f()
	}()

}

func getData(chan_i chan int, chan_f chan func()) {
	for i := range chan_i {
		fmt.Println(i)
	}
	chan_f <- func() {
		fmt.Println("end")
	}
}

func setData(chan_i chan int) {
	for i := 0; i < 10; i++ {
		chan_i <- i
	}
}
