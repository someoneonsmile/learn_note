// 我们可以使用通道来同步 Go 协程间的执行状态。这里是一个
// 使用阻塞的接受方式来等待一个 Go 协程的运行结束。

package main

import (
	"fmt"
	"math/rand"
	"time"
)

// 这是一个我们将要在 Go 协程中运行的函数。`done` 通道
// 将被用于通知其他 Go 协程这个函数已经工作完毕。
func worker(done chan bool, n int) {
	fmt.Print("working...")
	fmt.Println("done")
	fmt.Println(n == 0)
	// 发送一个值来通知我们已经完工啦。
	done <- n == 0
}

func main() {
	r := rand.New(rand.NewSource(time.Now().UnixNano()))

	// 运行一个 worker Go协程，并给予用于通知的通道。
	done := make(chan bool, 1)
	go worker(done, r.Intn(2))

	// 程序将在接收到通道中 worker 发出的通知前一直阻塞。
	for ret := <-done; !ret; ret = <-done {
		time.Sleep(3 * time.Second)
		fmt.Println("try again")
		go worker(done, r.Intn(2))
	}
}
