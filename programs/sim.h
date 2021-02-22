#ifndef LIBC_H
#define LIBC_H
#include <stdint.h>



void exit(uint32_t x);

uint32_t write(uint32_t _handle, const char *data, uint32_t size);

#endif /* LIBC_H */
