package main

import (
	"fmt"
	"time"
)

//func main() {
//	// NewTimer 创建一个 Timer，它会在最少过去时间段 d 后到期，向其自身的 C 字段发送当时的时间
//	timer1 := time.NewTimer(5 * time.Second)
//	fmt.Println("Start at: ", time.Now().Format("2006-01-02 15:04:05"))
//	go func(t *time.Timer) {
//		times := 0
//		for {
//			<-t.C
//			fmt.Println("timer", time.Now().Format("2006-01-02 15:04:05"))
//			times++
//			fmt.Println("reset timer's duration to 2s")
//			t.Reset(2 * time.Second)
//			if times > 3 {
//				fmt.Println("stop the timer")
//				t.Stop()
//			}
//		}
//	}(timer1)
//	time.Sleep(30 * time.Second)
//	fmt.Println("End at: ", time.Now().Format("2006-01-02 15:04:05"))
//}

type Timer struct {
	duration time.Duration
	done     chan bool
}

func NewTimer(duration time.Duration) *Timer {
	return &Timer{
		duration: duration,
		done:     make(chan bool),
	}
}

func (t *Timer) Start() {
	go func() {
		select {
		case <-time.After(t.duration):
			t.done <- true
		}
	}()
}

func (t *Timer) Stop() {
	t.done <- true
}

func main() {
	duration := 5 * time.Second // Set the duration for the timer

	fmt.Println("Starting timer...")

	timer := NewTimer(duration) // Create a new timer with the specified duration

	timer.Start() // Start the timer

	<-timer.done // Wait for the timer to expire

	fmt.Println("Timer expired!")
}
