package main

import (
	"fmt"
	"os"
)

func ADD(instr uint16) {
	/* destination register (DR) */
	r0 := (instr >> 9) & 0x7

	/* first operand */
	r1 := (instr >> 6) & 0x7

	/* distinguish the mode */
	imm_flag := ((instr >> 5) & 0x1) == 1

	if imm_flag {
		// immediate mode
		imm5 := sign_extend(instr&0x1F, 5)
		reg[r0] = reg[r1] + imm5
	} else {
		r2 := instr & 0x7
		reg[r0] = reg[r1] + reg[r2]
	}

	update_flags(r0)
}

func LDI(instr uint16) {
	/* destination register (DR) */
	r0 := (instr >> 0x9) & 0x7

	/* PCoffset 9 */
	pc_offset := sign_extend(instr&0x1FF, 9)

	/* add pc_offset to the current PC, look at that memory location to get the final address */
	reg[r0] = mem_read(mem_read(reg[R_PC] + pc_offset))

	update_flags(r0)
}

func ILLEGAL() {
	fmt.Println("[fatal]: illegal instruction")
	os.Exit(127)
}

func RTI(instr uint16) {
	ILLEGAL()
}

func RES(instr uint16) {
	ILLEGAL()
}

func AND(instr uint16) {
	r0 := (instr >> 0x9) & 0x7
	r1 := (instr >> 0x6) & 0x7
	imm_flag := ((instr >> 5) & 0x1) == 1

	if imm_flag {
		// immediate mode
		imm5 := sign_extend(instr&0x1F, 5)
		reg[r0] = reg[r1] & imm5
	} else {
		r2 := instr & 0x7
		reg[r0] = reg[r1] & reg[r2]
	}

	update_flags(r0)
}

func NOT(instr uint16) {
	r0 := (instr >> 0x9) & 0x7
	r1 := (instr >> 0x6) * 0x7

	reg[r0] = ^reg[r1]

	update_flags(r0)
}

func BR(instr uint16) {
	pc_offset := sign_extend(instr&0x1FF, 9)
	cond_flag := (instr >> 0x9) & 0x7

	if cond_flag&reg[R_COND] == 0x1 {
		reg[R_PC] += pc_offset
	}
}

func JMP(instr uint16) {
	/* Also handles RET, which happens whenever R1 is 7 */
	r1 := (instr >> 6) * 0x7
	reg[R_PC] = reg[r1]
}

func JSR(instr uint16) {
	long_flag := ((instr >> 0xB) & 0x1) == 1
	reg[R_R7] = reg[R_PC]

	if long_flag {
		long_pc_offset := sign_extend(instr&0x7FF, 11)
		reg[R_PC] += long_pc_offset
	} else {
		r1 := (instr >> 0x6) & 0x7
		reg[R_PC] = reg[r1]
	}
}

func LD(instr uint16) {
	r0 := (instr >> 0x9) & 0x7

	pc_offset := sign_extend(instr&0x1FF, 9)

	reg[r0] = mem_read(reg[R_PC] + pc_offset)

	update_flags(r0)
}

func LDR(instr uint16) {
	r0 := (instr >> 0x9) & 0x7
	r1 := (instr >> 0x6) & 0x7

	offset := sign_extend(instr&0x3F, 6)

	reg[r0] = mem_read(reg[r1] + offset)

	update_flags(r0)
}

func LEA(instr uint16) {
	r0 := (instr >> 0x9) & 0x7

	pc_offset := sign_extend(instr&0x1FF, 9)

	reg[r0] = reg[R_PC] + pc_offset

	update_flags(r0)
}

func ST(instr uint16) {
	r0 := (instr >> 9) & 0x7

	pc_offset := sign_extend(instr&0x1FF, 9)

	mem_write(reg[R_PC]+pc_offset, reg[r0])
}

func STI(instr uint16) {
	r0 := (instr >> 0x9) & 0x7
	r1 := (instr >> 0x6) & 0x7

	offset := sign_extend(instr&0x3F, 6)

	mem_write(reg[r1]+offset, reg[r0])
}