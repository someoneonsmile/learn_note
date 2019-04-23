package main

import (
	"fmt"
	"math/rand"
)

func main() {
	numbers := make([]int, 100000)
	for i := 0; i < 100000; i++ {
		numbers[i] = rand.Intn(100)
	}
	//done := make(chan int)
	fmt.Println(numbers)
	quickSort(numbers, 0, len(numbers)-1)
	fmt.Println(numbers)
}

func quickSort(values []int, left int, right int) {
	if left < right {

		// 设置分水岭
		temp := values[left]

		// 设置哨兵
		i, j := left, right
		for {
			// 从右向左找，找到第一个比分水岭小的数
			for values[j] >= temp && i < j {
				j--
			}

			// 从左向右找，找到第一个比分水岭大的数
			for values[i] <= temp && i < j {
				i++
			}

			// 如果哨兵相遇，则退出循环
			if i >= j {
				break
			}

			// 交换左右两侧的值
			values[i], values[j] = values[j], values[i]
		}

		// 将分水岭移到哨兵相遇点
		values[left] = values[i]
		values[i] = temp

		// 递归，左右两侧分别排序
		quickSort(values, left, i-1)

		quickSort(values, i+1, right)
	}
}

func quickSort2(values []int, left int, right int, done chan int) {
	if left < right {

		// 设置分水岭
		temp := values[left]

		// 设置哨兵
		i, j := left, right
		for {
			// 从右向左找，找到第一个比分水岭小的数
			for values[j] >= temp && i < j {
				j--
			}

			// 从左向右找，找到第一个比分水岭大的数
			for values[i] <= temp && i < j {
				i++
			}

			// 如果哨兵相遇，则退出循环
			if i >= j {
				break
			}

			// 交换左右两侧的值
			values[i], values[j] = values[j], values[i]
		}

		// 将分水岭移到哨兵相遇点
		values[left] = values[i]
		values[i] = temp

		// 递归，左右两侧分别排序
		go quickSort2(values, left, i-1, done)

		go quickSort2(values, i+1, right, done)
		<-done
		<-done
	}

	done <- 0
}
