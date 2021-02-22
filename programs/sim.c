#include "sim.h"

#define SYS_EXIT ( 0x01 )
#define SYS_WRITE ( 0x02 )

void exit(uint32_t x) {
    asm volatile( "mov r0, %1 \n" // assign r0 =  x
                  "svc %0     \n" // make system call SYS_EXIT
    :
    : "I" (SYS_EXIT), "r" (x)
    : "r0" );
}

uint32_t write(uint32_t _handle, const char *data, uint32_t size) {
    int r;
    asm volatile( "mov r0, %2 \n" // assign r0 = data
                  "mov r1, %3 \n" // assign r1 =  size
                  "svc %1     \n" // make system call SYS_WRITE
                  "mov %0, r0 \n" // assign r  = r0
    : "=r" (r)
    : "I" (SYS_WRITE), "r" (data), "r" (size)
    : "r0", "r1", "r2" );
    return r;
}
