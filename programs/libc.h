#ifndef LIBC_H
#define LIBC_H
#include <stdint.h>

#define SYS_EXIT ( 0x01 )

extern void exit(uint32_t x);

#endif /* LIBC_H */
