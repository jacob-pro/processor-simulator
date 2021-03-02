#include "sim.h"
#include <stdio.h>
#include <stdint.h>

static uint64_t input = 93;

uint64_t fibonnaci(uint64_t n) {
    if (n == 0) return 0;
    if (n == 1) return 1;

    uint64_t n1 = 0;
    uint64_t n2 = 1;
    uint64_t n3 = 0;

    for(uint64_t i = 1; i < n; i++) {
        if (n2 > UINT64_MAX - n1) {
            printf("Integer overflow on %llu", i + 1);
            exit(EXIT_FAILURE);
        }
        n3 = n1 + n2;
        n1 = n2;
        n2 = n3;
    }
    return n3;
}

int main() {

    uint64_t result = fibonnaci(input);
    printf("Fibonacci %llu = %llu", input, result);

    return EXIT_SUCCESS;
}
