#include <stdio.h>
#include <time.h>
#include <stdlib.h>
#include <unistd.h>

int main() {
    // print the pid of this process
    printf("pid: %d\n", getpid());
    while (1) {
        // get current date and time then print it
        time_t now = time(NULL);
        printf("Current date and time: %s", ctime(&now));
        sleep(1);
    }
    return 0;
}