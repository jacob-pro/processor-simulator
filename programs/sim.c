#include "sim.h"

#ifdef NOSTDLIB

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wincompatible-library-redeclaration"
int strlen(const char * str) {
    int counter = 0;
    while (str[counter] != '\0') {
        counter++;
    }
    return counter;
}
#pragma clang diagnostic pop

#else
#include <sys/stat.h>
#include <string.h>
#endif

#define SYS_EXIT  1
#define SYS_WRITE 2

// https://interrupt.memfault.com/blog/boostrapping-libc-with-newlib#system-calls

void _exit(int status) {
    asm volatile( "mov r0, %1 \n" // assign r0 =  status
                  "svc %0     \n" // make system call SYS_EXIT
    :
    : "I" (SYS_EXIT), "r" (status)
    : "r0" );
}

int _write(int fd, char *buf, int count) {
    int r;
    asm volatile( "mov r0, %2 \n" // assign r0 = buf
                  "mov r1, %3 \n" // assign r1 =  count
                  "svc %1     \n" // make system call SYS_WRITE
                  "mov %0, r0 \n" // assign r  = r0
    : "=r" (r)
    : "I" (SYS_WRITE), "r" (buf), "r" (count)
    : "r0", "r1", "r2" );
    return r;
}

inline void write_str(char *str) {
    _write(0, str, strlen(str));
}

inline void write(char *buf, int count) {
    _write(0, buf, count);
}

#ifndef NOSTDLIB

int _read (int fd, char *buf, int count) {
    write_str("Error: _read unimplemented\n");
    _exit(EXIT_FAILURE);
}

int _close(int file) {
    return -1;
}

#define MAX_HEAP_SIZE 4096
static char HEAP[MAX_HEAP_SIZE];

void *_sbrk(int incr) {
    static char* heap = &HEAP[0];
    char *prev_heap = heap;
    if (heap > &HEAP[MAX_HEAP_SIZE]) {
        char *msg = "OUT OF HEAP SPACE\n";
        _write(0, msg, strlen(msg));
        _exit(EXIT_FAILURE);
    }
    return prev_heap;
}

int _fstat(int file, struct stat *st) {
    st->st_mode = S_IFCHR;
    return 0;
}

int _isatty(int file) {
    return 1;   // File is character device (a terminal, console, printer, or serial port).
}

int _lseek(int file, int ptr, int dir) {
    write_str("Error: _lseek unimplemented\n");
    _exit(EXIT_FAILURE);
}

void _kill(int pid, int sig) {
    write_str("Error: _kill unimplemented\n");
    _exit(EXIT_FAILURE);
}

int _getpid(void) {
    write_str("Error: _getpid unimplemented\n");
    _exit(EXIT_FAILURE);
}

void _assert_impl(bool x, char *file, int line) {
    if (!(x)) {
        char buffer[33];
        itoa(line, buffer, 10);
        write_str("Assertion Failed: ");
        write_str(file);
        write_str(" line ");
        write_str(buffer);
        write_str("\n");
        exit(EXIT_FAILURE);
    }
}

#endif




