#include <stdio.h>
#include "sim.h"
#include <string.h>
#include <stdbool.h>

void assert_true(bool condition, char* msg) {
    if (!condition) {
        write_str("Assertion Failed: ");
        write_str(msg);
        write_str("\n");
        exit(1);
    }
}

void assert_false(bool condition, char* msg) {
    assert_true(!condition, msg);
}


int main() {

    int x = 5;
    int y = 10;
//    assert_true(x < y, "x < y");
//    assert_false(x > y, "x > y");
//    assert_true(x != y, "x != y");
//    assert_true(x == x, "x == x");

    int b = x - y;
    exit(b);

    assert_true(b == -5, "x - y");
    assert_true(y - x == 5, "y - x");
    assert_true(y + x == 15, "y + x");


//    printf("printf \n %d \n", 5);
//
//
//    char hello[] = {'H', 'E', 'L', 'L', 'O', '\n'};
//    write(hello, sizeof(hello));
//
//
//    int a[] = {29, 2, 33, 4, 54};
//    int b[] = {29, 2, 34, 45, 54};
//
//    for (int i = 0; i < 5; i++) {
//        a[i] = a[i] + b[i];
//    }

    exit(25);
    return 25;
}


