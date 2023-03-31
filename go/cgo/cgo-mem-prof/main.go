package main

import (
	"fmt"
	"runtime"
	"sync"
	"sync/atomic"
)

func main() {
	fmt.Println(runtime.NumCPU())
	a := int32(0)
	wg := sync.WaitGroup{}
	wg.Add(2)
	subFunc := func() {
		defer wg.Done()
		for i := 0; i < 10000; i++ {
			if a == a {
				atomic.AddInt32(&a, 1)
			}
		}
	}
	go subFunc()
	go subFunc()
	wg.Wait()
	fmt.Println(a)
}
