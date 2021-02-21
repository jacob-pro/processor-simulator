#include "sim.h"

void exit(uint32_t x) {
    asm volatile( "mov r0, %1 \n" // assign r0 =  x
                  "svc %0     \n" // make system call SYS_EXIT
    :
    : "I" (SYS_EXIT), "r" (x)
    : "r0" );
}
