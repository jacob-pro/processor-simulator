#include "sim.h"

int main() {

    int a[] = {29, 2, 33, 4, 54};
    int b[] = {29, 2, 34, 45, 54};

    for (int i = 0; i < 5; i++) {
        a[i] = a[i] + b[i];
    }

    exit(5);
    return 5;
}
