package main

/*
#include <stdio.h>
*/
import "C"
import (
	"bufio"
	"fmt"
)

const (
	TRAP_GETC  uint8 = 0x20 /* get character from keyboard, not echoed onto the terminal */
	TRAP_OUT         = 0x21 /* output a character */
	TRAP_PUTS        = 0x22 /* output a word string */
	TRAP_IN          = 0x23 /* get character from keyboard, echoed onto the terminal */
	TRAP_PUTSP       = 0x24 /* output a byte string */
	TRAP_HALT        = 0x25 /* halt the program */
)

var reader *bufio.Reader
var writer *bufio.Writer

func _TRAP_GETC() {
	/* read a single ASCII char */
	reg[R_R0] = uint16(C.getchar())
	update_flags(R_R0)
}

func _TRAP_OUT() {
	C.putc(C.int(reg[R_R0]), C.stdout)
	C.fflush(C.stdout)
}

func _TRAP_PUTS() {
	loc := reg[R_R0]
	for {
		c := byte(memory[loc])
		loc++
		if c == 0 {
			break
		}
		C.putc(C.int(c), C.stdout)
	}
	C.fflush(C.stdout)
}

func _TRAP_IN() {
	fmt.Print("Enter a character: ")
	c := C.getchar()
	C.putc(c, C.stdout)
	C.fflush(C.stdout)
	reg[R_R0] = uint16(c)
	update_flags(R_R0)
}

func _TRAP_PUTSP() {
	/* one char per byte (two bytes per word)
	   here we need to swap back to
	   big endian format */
	loc := reg[R_R0]
	for {
		if memory[loc] == 0 {
			break
		}

		c1 := byte(memory[loc] & 0xFF)
		C.putc(C.int(c1), C.stdout)

		c2 := byte(memory[loc] >> 8)
		if c2 != 0 {
			C.putc(C.int(c2), C.stdout)
		}

		loc++
	}
	C.fflush(C.stdout)
}

func _TRAP_HALT() {
	fmt.Println("HALT")
	running = false
}
