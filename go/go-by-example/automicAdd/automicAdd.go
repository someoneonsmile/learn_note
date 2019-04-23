// automicAdd
package main

import (
	"fmt"
	"math/rand"
	"runtime"
	"sync/atomic"
	"time"
)

func main() {

	qsize := 50
	queue := make(chan bool, 1)

	var ops uint64 = 0
	for i := 1; i <= qsize; i++ {
		go func() {
			atomic.AddUint64(&ops, 1)
			time.Sleep(time.Second * time.Duration(rand.Intn(10)))
			queue <- true
			runtime.Gosched()
		}()
	}
	for i := 1; i <= qsize; i++ {
		fmt.Println(time.Now())
		<-queue
	}
	opsFinal := atomic.LoadUint64(&ops)
	fmt.Println(opsFinal)

}
