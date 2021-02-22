#include <sys/stat.h>

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

int _read (int fd, char *buf, int count) {
    _exit(10);
}

int _close(int file) {
    _exit(11);
}

void *_sbrk(int incr) {
    _exit(12);
}

int _fstat(int file, struct stat *st) {
    st->st_mode = S_IFCHR;
    return 0;
}

int _isatty(int file) {
    _exit(14);
}

int _lseek(int file, int ptr, int dir) {
    _exit(15);
}

void _kill(int pid, int sig) {
    _exit(16);
}

int _getpid(void) {
    _exit(17);
}
