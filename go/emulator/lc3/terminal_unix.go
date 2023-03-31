//go:build !windows
// +build !windows

package main

import (
	"fmt"
	"github.com/pkg/term/termios"
	"golang.org/x/sys/unix"
	"os"
	"os/signal"
)

func check_key() uint16 {
	var readfds unix.FdSet

	var timeout unix.Timeval
	timeout.Sec = 0
	timeout.Usec = 0

	readfds.Zero()
	readfds.Set(unix.Stdin)

	n, err := unix.Select(1, &readfds, nil, nil, &timeout)
	if err != nil {
		return uint16(n)
	} else {
		return 0
	}
}

var original_tio unix.Termios

func disable_input_buffering() {
	tmp_tio, _ := termios.Tcgetattr(uintptr(unix.Stdin))
	original_tio = *tmp_tio
	new_tio := original_tio
	//fmt.Printf("%x\n", new_tio.Lflag & uint64(-265))
	//new_tio.Lflag &= ^termios.ICANON & ^termios.ECHO // -101 & -9
	new_tio.Lflag = 0x4c3

	termios.Tcsetattr(uintptr(unix.Stdin), termios.TCSANOW, &new_tio)
}

func restore_input_buffering() {
	termios.Tcsetattr(uintptr(unix.Stdin), termios.TCSANOW, &original_tio)
}

func handle_interrupt(signal int) {
	restore_input_buffering()
	fmt.Println()
	os.Exit(-2)
}

func setup_sigint_handler() {
	restore_input_buffering()
	c := make(chan os.Signal)
	signal.Notify(c, os.Interrupt)
	go func() {
		<-c
		fmt.Println("\r- Ctrl+C pressed in Terminal")
		os.Exit(-2)
	}()
}
