#include "libc.h"

void start() {

    int a[] = {1, 2, 3, 4, 5};
    int b[] = {1, 2, 3, 4, 5};
    int c[] = {1, 2, 3, 4, 5};

    for (int i = 0; i < 5; i++ ) {
        a[i] = b[i] + c[i];
    }

    asm volatile( "mov r0, %1 \n" // assign r0 =  x
                  "svc %0     \n" // make system call SYS_EXIT
    :
    : "I" (SYS_EXIT), "I" (241)
    : "r0" );

}
