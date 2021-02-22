#include <stdio.h>

int main() {

    char hello[] = {'H', 'E', 'L', 'L', 'O', '\n'};
    _write(0, hello, sizeof(hello));
//    printf("HELLO2");


    int a[] = {29, 2, 33, 4, 54};
    int b[] = {29, 2, 34, 45, 54};

    for (int i = 0; i < 5; i++) {
        a[i] = a[i] + b[i];
    }

    return 25;
}
