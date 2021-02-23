#ifndef SIM_H
#define SIM_H
#include <stdlib.h>

#define EXIT_SUCCESS 0
#define EXIT_FAILURE 1


void write(char *buf, int count);

void write_str(char *buf);

#define assert(x) (                  \
    {if (!(x)) {                     \
    char buffer[33];                 \
    itoa(__LINE__, buffer, 10);      \
    write_str("Assertion Failed: "); \
    write_str(__FILE__);             \
    write_str(" line ");             \
    write_str(buffer);               \
    write_str("\n");                 \
    exit(EXIT_FAILURE);              \
    }}  )


#endif /* SIM_H */
