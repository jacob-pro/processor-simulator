#include "sim.h"
#include <sys/stat.h>
#include <string.h>

#define SYS_EXIT ( 0x01 )
#define SYS_WRITE ( 0x02 )

// https://interrupt.memfault.com/blog/boostrapping-libc-with-newlib#system-calls

void _exit(int status) {
    asm volatile( "mov r0, %1 \n" // assign r0 =  status
                  "svc %0     \n" // make system call SYS_EXIT
    :
    : "I" (SYS_EXIT), "r" (status)
    : "r0" );
}

inline void exit(int status) {
    _exit(status);
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

int _read (int fd, char *buf, int count) {
    char *msg = "Error: _read unimplemented\n";
    _write(0, msg, strlen(msg));
    _exit(1);
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
        _exit(1);
    }
    return prev_heap;
}

int _fstat(int file, struct stat *st) {
    st->st_mode = S_IFCHR;
    return 0;
}

int _isatty(int file) {
    char *msg = "Error: _isatty unimplemented\n";
    _write(0, msg, strlen(msg));
    _exit(1);
}

int _lseek(int file, int ptr, int dir) {
    char *msg = "Error: _lseek unimplemented\n";
    _write(0, msg, strlen(msg));
    _exit(1);
}

void _kill(int pid, int sig) {
    char *msg = "Error: _kill unimplemented\n";
    _write(0, msg, strlen(msg));
    _exit(1);
}

int _getpid(void) {
    char *msg = "Error: _getpid unimplemented\n";
    _write(0, msg, strlen(msg));
    _exit(1);
}
