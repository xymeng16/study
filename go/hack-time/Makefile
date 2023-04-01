CC=gcc
CFLAGS=-fPIE -O2
GO=go

fakeclock/fake_gettimeofday.o: fakeclock/fake_gettimeofday.c
	$(CC) -c fakeclock/fake_gettimeofday.c $(CFLAGS) -o fakeclock/fake_gettimeofday.o

fakeclock/fake_clock_gettime.o: fakeclock/fake_clock_gettime.c
	$(CC) -c fakeclock/fake_clock_gettime.c $(CFLAGS) -o fakeclock/fake_clock_gettime.o

timer: test/bin/main.go
	$(GO) build -o test/bin/timer ./test/bin/main.go

test: fakeclock/fake_gettimeofday.o fakeclock/fake_clock_gettime.o timer
	$(GO) test