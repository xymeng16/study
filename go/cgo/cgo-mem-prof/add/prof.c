#include <stdio.h>
#include <time.h>
#include <stdlib.h>

struct A {
    void *small;
    void *large;
};

#define FACTOR (100000)

int main()
{
    FILE *f = fopen("/dev/random", "r");
    unsigned int seed;
    fread(&seed, sizeof(seed), 1, f);
    fclose(f);
    srand(seed);

    double r = ((double)rand()/(double)(RAND_MAX)) / 2 + 0.5;
    int small = (int)(FACTOR * r);
    int large = small * 100;
    printf("small: %d, large: %d\n", small, large);

    struct A obj;
    obj.small = malloc(small);
    obj.large = malloc(large);


    free(obj.small);
    free(obj.large);
    
    return 0;
}