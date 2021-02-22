#include "sim.h"
#include <stdio.h>

int main() {

//    int a[] = {29, 2, 33, 4, 54};
    char a[] = {'a', 'b', 'c', 'd', '\n'};
    int b[] = {29, 2, 34, 45, 54};

//    for (int i = 0; i < 5; i++) {
//        a[i] = a[i] + b[i];
//    }

//    memset(&b[0], 23, 5);
//    printf("%d", 5);
    write(0, a, 5);

    exit(b[2]);
}

int __stack = 0;
