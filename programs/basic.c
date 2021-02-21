#include "libc.h"

void start() {

    int a[] = {29, 2, 33, 4, 54};
    int b[] = {29, 2, 34, 45, 54};

    for (int i = 0; i < 5; i++) {
        a[i] = a[i] + b[i];
    }

    exit(a[3]);

}
