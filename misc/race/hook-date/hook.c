#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <sys/ptrace.h>
#include <sys/wait.h>
#include <sys/user.h>
#include <unistd.h>

int main(int argc, char **argv) {
    if (argc != 2) {
        fprintf(stderr, "Usage: %s pid\n", argv[0]);
        return 1;
    }

    pid_t pid = atoi(argv[1]);
    if (ptrace(PTRACE_ATTACH, pid, NULL, NULL) == -1) {
        perror("PTRACE_ATTACH: ");
        return 1;
    }

    if (waitpid(pid, NULL, 0) == -1) {
        perror("waitpid");
        return 1;
    }

    struct user_regs_struct regs;
    if (ptrace(PTRACE_GETREGS, pid, NULL, &regs) == -1) {
        perror("PTRACE_GETREGS: ");
        return 1;
    }

    long time_addr = regs.rax; // rax contains the result of the time() function
    long new_time = time(NULL) + 3600; // add 1 hour
    if (ptrace(PTRACE_POKEDATA, pid, time_addr, new_time) == -1) {
        perror("PTRACE_POKEDATA: ");
        return 1;
    }

    if (ptrace(PTRACE_DETACH, pid, NULL, NULL) == -1) {
        perror("PTRACE_DETACH: ");
        return 1;
    }

    return 0;
}