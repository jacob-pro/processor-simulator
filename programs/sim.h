#ifndef SIM_H
#define SIM_H
#include <stdlib.h>
#include <stdbool.h>

#define EXIT_SUCCESS 0
#define EXIT_FAILURE 1

void write(char *buf, int count);

void write_str(char *buf);

void _assert_impl(bool x, char *file, int line);

#define assert(x) _assert_impl((x), __FILE__, __LINE__)

#endif /* SIM_H */
