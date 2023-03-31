//go:build windows
// +build windows

package main

/*
#include <conio.h>
int kbhit_go() {
	return _kbhit();
}
*/
import "C"

import (
	"fmt"
	"golang.org/x/sys/windows"
	"os"
	"os/signal"
)

func _kbhit() bool {
	ret := int(C.kbhit_go())
	if ret != 0 {
		return true
	} else {
		return false
	}
}

func check_key() bool {
	hStdin, _ := windows.GetStdHandle(windows.STD_INPUT_HANDLE)
	result, _ := windows.WaitForSingleObject(hStdin, 1000)
	return (result == windows.WAIT_OBJECT_0) && _kbhit()
}

var fdwMode, fdwOldMode uint32

func disable_input_buffering() {
	hStdin, _ := windows.GetStdHandle(windows.STD_INPUT_HANDLE)
	windows.GetConsoleMode(hStdin, &fdwOldMode)
	fdwMode = fdwOldMode ^ windows.ENABLE_ECHO_INPUT ^ windows.ENABLE_LINE_INPUT
	windows.SetConsoleMode(hStdin, fdwMode)
	windows.FlushFileBuffers(hStdin)
}

func restore_input_buffering() {
	hStdin, _ := windows.GetStdHandle(windows.STD_INPUT_HANDLE)
	windows.SetConsoleMode(hStdin, fdwOldMode)
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
