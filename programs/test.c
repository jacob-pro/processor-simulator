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

    write_str("Starting logic tests...\n");

    {
        int x = -5;
        int y = 10;
        assert_true(x < y, "x < y");
        assert_false(x > y, "x > y");
        assert_true(x != y, "x != y");
        assert_true(x == x, "x == x");
    }
    {
        unsigned int x = 5;
        unsigned int y = 10;
        assert_true(x < y, "x < y");
        assert_false(x > y, "x > y");
        assert_true(x != y, "x != y");
        assert_true(x == x, "x == x");
    }
    {
        int x = 0xFF;
        int y = 0xFF;
        int b = (x == y);
        int c = (x != y);
        assert_true(b == 1, "b == 1");
        assert_true(c == 0, "c == 0");
    }


    printf("printf %d \n", 5);

    return 0;
}


