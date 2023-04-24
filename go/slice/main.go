package main

import "fmt"

func print(a []int) {
	fmt.Println("in func: ", a)
}

func main() {
	a := []int{1, 2, 3, 4, 5, 6}
	b := a
	fmt.Println("a:", a)
	fmt.Println("b:", b)
	print(a)
	// remove 3 from a
	a = append(a[:2], a[3:]...)
	fmt.Println("a:", a)
	fmt.Println("b:", b)
	print(a)
	// remove 1 from a
	a = append(a[:0], a[1:]...)
	fmt.Println("a:", a)
	fmt.Println("b:", b)
	print(a)
}
