package main

import (
	"fmt"
	"time"
)

func main() {

	// 模拟五个请求
	reqQueue := make(chan int, 5)
	for i := 1; i <= 5; i++ {
		reqQueue <- i
	}
	// 关闭通道
	close(reqQueue)

	// 打点器
	ticker := time.NewTicker(time.Millisecond * 500)
	for r := range reqQueue {
		<-ticker.C
		fmt.Println(r, time.Now())
	}

	// 定义初始桶大小
	rateLimiter := make(chan time.Time, 5)
	// 添加五个令牌
	for i := 1; i <= 5; i++ {
		rateLimiter <- time.Now()
	}
	// 令牌速率
	go func() {
		ticker := time.NewTicker(time.Millisecond * 200)
		for r := range ticker.C {
			rateLimiter <- r
		}
	}()

	// 模拟五个请求
	reqChan := make(chan int, 15)
	for i := 1; i <= 15; i++ {
		reqChan <- i
	}
	// 关闭通道
	close(reqChan)

	// 遍历通道信息
	for req := range reqChan {
		<-rateLimiter
		fmt.Println(req, time.Now())
	}

}
