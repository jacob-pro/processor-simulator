#include <stdint.h>
#include "sim.h"

#ifndef UNROLLED
int bitcount(uint64_t n)
{
    int bits = 0;
    while (n != 0)
    {
        if (n & 1) bits++;
        n >>= 1;
    }
    return bits;
}
#else
int bitcount(uint64_t n)
{
    int bits = 0;
    while (n != 0)
    {
        if (n & 1) bits++;
        if (n & 2) bits++;
        if (n & 4) bits++;
        if (n & 8) bits++;
        n >>= 4;
    }
    return bits;
}
#endif

void start() {
    uint64_t value = 3512030540234565643;
    exit(bitcount(value));
}
