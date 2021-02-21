#include "sim.h"

int main() {

    int a[] = {29, 2, 33, 4, 54};
    int b[] = {29, 2, 34, 45, 54};

    for (int i = 0; i < 5; i++) {
        a[i] = a[i] + b[i];
    }

    memset(&b, 23, 5);

    exit(b[4]);
}

int __stack = 0;
