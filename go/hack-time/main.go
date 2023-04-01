package main

import (
	"fmt"
	"hack-time/log"
	"os"
	"strconv"
)

func main() {
	// usage: ./hack-time [pid] [delta_sec] [delta_nsec]
	// read parameters
	if len(os.Args) != 4 {
		fmt.Println("usage: ./hack-time [pid] [delta_sec] [delta_nsec]")
		return
	}
	pid, err := strconv.Atoi(os.Args[1])
	if err != nil {
		fmt.Println("error parsing pid:", err)
		return
	}
	deltaSec, err := strconv.ParseInt(os.Args[2], 10, 64)
	if err != nil {
		fmt.Println("error parsing delta_sec:", err)
		return
	}
	deltaNSec, err := strconv.ParseInt(os.Args[3], 10, 64)
	if err != nil {
		fmt.Println("error parsing delta_nsec:", err)
		return
	}

	// get all pids of child processes/threads from /proc/[pid]/task
	taskDir := "/proc/" + strconv.Itoa(pid) + "/task"
	taskDirFd, err := os.Open(taskDir)
	defer taskDirFd.Close()
	if err != nil {
		fmt.Println("error opening task dir:", err)
		return
	}

	pids, err := taskDirFd.Readdirnames(0)
	if err != nil {
		fmt.Println("error reading task dir:", err)
		return
	}

	// hack time for each pid
	logger, err := log.NewDefaultZapLogger()
	if err != nil {
		fmt.Println("error creating logger:", err)
		return
	}

	s, err := GetSkew(logger, NewConfig(deltaSec, deltaNSec, 1))
	if err != nil {
		fmt.Println("error creating skew:", err)
		return
	}

	for _, pid := range pids {
		pidInt, err := strconv.Atoi(pid)
		if err != nil {
			fmt.Println("error parsing pid:", err)
			return
		}
		err = s.Inject(pidInt)
		if err != nil {
			fmt.Println("error injecting skew for pid #", pid, ":", err)
			return
		}
	}
}
