package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
)

/* 65536 memory locations */
var memory [math.MaxUint16]uint16

/* 10 registers */
const (
	R_R0 uint16 = iota
	R_R1
	R_R2
	R_R3
	R_R4
	R_R5
	R_R6
	R_R7
	R_PC /* program counter */
	R_COND
	R_COUNT
)

var reg [R_COUNT]uint16

/* 16 opcodes */
const (
	OP_BR   uint16 = iota /* branch */
	OP_ADD                /* add  */
	OP_LD                 /* load */
	OP_ST                 /* store */
	OP_JSR                /* jump register */
	OP_AND                /* bitwise and */
	OP_LDR                /* load register */
	OP_STR                /* store register */
	OP_RTI                /* unused */
	OP_NOT                /* bitwise not */
	OP_LDI                /* load indirect */
	OP_STI                /* store indirect */
	OP_JMP                /* jump */
	OP_RES                /* reserved (unused) */
	OP_LEA                /* load effective address */
	OP_TRAP               /* execute trap */
)

/* condition flags */
const (
	FL_POS uint16 = 1 << 0 /* positive */
	FL_ZRO        = 1 << 1 /* zero */
	FL_NEG        = 1 << 2 /* negative */
)

var running bool = false

func main() {
	/* load arguments */
	args := os.Args
	if len(args) < 2 {
		/* show usage string */
		fmt.Println("usage: lc3 [image-file1] ...")
		os.Exit(2)
	}

	for j := 1; j < len(args); j++ {
		if !read_image(args[j]) {
			fmt.Printf("failed to load image %s\n...", args[j])
			os.Exit(1)
		}
	}

	/* setup terminal */
	setup_sigint_handler()
	disable_input_buffering()

	/* initialize the standard stream reader and writer */
	reader = bufio.NewReader(os.Stdin)
	writer = bufio.NewWriter(os.Stdout)

	/* since exactly one condition flag should be set at any given time, set the Z flag */
	reg[R_COND] = FL_ZRO

	/* set the PC to starting position */
	/* 0x3000 is the default */
	const PC_START = 0x3000
	reg[R_PC] = PC_START

	running = true
	for running {
		/* FETCH */
		instr := mem_read(reg[R_PC])
		op = instr >> 12
		reg[R_PC]++

	}
}

func _(op uint16) {

}
