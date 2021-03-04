#include "sim.h"
#include <stdio.h>
#include <stdint.h>
#include <stdbool.h>

static uint64_t input = 20;
static bool TERMINATE_ON_OVERFLOW = true;

uint64_t factorial(uint64_t n) {
    if (n == 0) {
        return 1;
    } else {
        uint64_t lower = factorial(n - 1);
        uint64_t ret = n * lower;
        if (TERMINATE_ON_OVERFLOW && ret / n != lower) {
            printf("Overflow on n=%llu", n);
            exit(EXIT_FAILURE);
        }
        return ret;
    }
}

int main() {
    uint64_t result = factorial(input);
    printf("Factorial (recursive) %llu = %llu", input, result);
    return EXIT_SUCCESS;
}
