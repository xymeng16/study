#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <pthread.h>
#include <stdint.h>
#include <unistd.h>
#include <sched.h>

volatile int a;

pthread_mutex_t mutex;
pthread_barrier_t barrier;

int ready = 0;

#define set_affinity(x) do { \
    cpu_set_t cpuset; \
    CPU_ZERO(&cpuset); \
    CPU_SET(x, &cpuset); \
    pthread_setaffinity_np(pthread_self(), sizeof(cpu_set_t), &cpuset); \
} while(0)

void f1() {
    set_affinity(0);
    pthread_barrier_wait(&barrier);
    a = 10;
    printf("f1: a = %d\n", a);

}

void f2() {
    set_affinity(11);
    pthread_barrier_wait(&barrier);
    a = 20;
    printf("f2: a = %d\n", a);

}

int main() {
    pthread_mutex_init(&mutex, NULL);
    pthread_barrier_init(&barrier, NULL, 2);

    pthread_t t1, t2;

    pthread_create(&t1, NULL, (void *(*)(void *)) f1, NULL);
    pthread_create(&t2, NULL, (void *(*)(void *)) f2, NULL);

//    cpu_set_t cpuset;
//
//    CPU_ZERO(&cpuset);
//    CPU_SET(0, &cpuset);
//    pthread_setaffinity_np(t1, sizeof(cpu_set_t), &cpuset);
//
//    CPU_ZERO(&cpuset);
//    CPU_SET(2, &cpuset);
//    pthread_setaffinity_np(t2, sizeof(cpu_set_t), &cpuset);

    pthread_join(t1, NULL);
    pthread_join(t2, NULL);
}