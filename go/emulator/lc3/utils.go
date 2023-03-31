package main

import (
	"encoding/binary"
	"fmt"
	"math"
	"os"
)

func sign_extend(x uint16, bit_count int) uint16 {
	if ((x >> (bit_count - 1)) & 1) == 1 {
		x |= 0xFFFF << bit_count
	}
	return x
}

func update_flags(r uint16) {
	if reg[r] == 0 {
		reg[R_COND] = FL_ZRO
	} else if ((reg[r] >> 15) & 1) == 1 {
		reg[R_COND] = FL_NEG
	} else {
		reg[R_COND] = FL_POS
	}
}

func mem_read(u uint16) uint16 {
	return u
}

func mem_write(addr uint16, val uint16) {

}

func swap16(x uint16) uint16 {
	return (x << 8) | (x >> 8)
}

func read_image_file(f *os.File) {
	/* the origin tells us where in memory to place the image */
	var origin_buff [2]byte /* The first 16-bit */
	//binary.BigEndian.PutUint16()
	read, err := f.Read(origin_buff[:])
	if err != nil {
		fmt.Printf("image location read error, expect: 2, got: %d\n", read)
		return
	}
	origin := binary.BigEndian.Uint16(origin_buff[:])

	/* we know the maximum file size, so we only need one fread */
	max_read := math.MaxUint16 - origin

}
func read_image(f string) bool {

	return false
}
