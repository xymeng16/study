// idea and some code borrowed from chaos-mesh

package main

import (
	"fmt"
	"github.com/go-logr/logr"
	"github.com/pkg/errors"
	"hack-time/mapreader"
	"hack-time/ptrace"
	"os"
	"runtime"
)

// try to extract the address of __vdso_clock_gettime in vDSO
// and replace it with the customized one

/*
 * the signature of __vdso_clock_gettime is:
 * int __vdso_clock_gettime(clockid_t clock, struct timespec *ts);
 * retrieves the time of the specific clock `clockid`, saving in
 * `ts`, defined as
 * 		struct timespec {
 * 			time_t   tv_sec;
 * 			long     tv_nsec;
 * 		};
 *
 */

// Params
// in linux, time_t is defined as long and clockid_t is defined as int
// in typical x64 machine sizeof(long) is 8, hence int64 is used
type Params struct {
	clockid         int
	diffSeconds     int64
	diffNanoSeconds int64
}

const CLOCK_REALTIME = 0
const CLOCK_MONOTIME = 1

func hackTime(pid int, params Params, logger logr.Logger) (err error) {
	runtime.LockOSThread()
	defer func() {
		runtime.UnlockOSThread()
	}()

	trace, err := ptrace.Trace(pid, logger)
	if err != nil {
		return errors.Wrapf(err, "failed to attach to #%d\n", pid)
	}
	defer func() {
		err = trace.Detach()
		if err != nil {
			logger.Error(err, "failed to detach from #%d\n", pid)
		}
	}()

	// find the corresponding vDSO area in the process
	var vDSOEntry *mapreader.Entry
	l := len(trace.Entries)
	for i := range trace.Entries {
		/* 	vDSO usually resides in entries with higher addresses
		(validated by `cat /proc/[pid]/maps)
		*/
		e := trace.Entries[l-i-1]
		if e.Path == "[vdso]" {
			vDSOEntry = &e
			break
		}
	}
	if vDSOEntry != nil {
		return errors.Wrapf(err, "cannot find [vdso] entry in $%d", pid)
	}

	// check if this process has already been hacked, if not, mmap the customized
	// `clock_gettime` to the process's VM region (with flag MAP_ANONYMOUS)

}

func main() {
	args := os.Args

	if len(args) != 4 {
		fmt.Errorf("usage: sudo %s [pid] [diffSeconds] [diffNanoSeconds]")
		os.Exit(-1)
	}

	params := Params{
		clockid:         CLOCK_REALTIME,
		diffSeconds:     0,
		diffNanoSeconds: 0,
	}

}
