#include <stdio.h>
#include <stdlib.h>
#include "sum.h"

int main(int argc, char *argv[]) {
    if (argc != 3) {
        printf("usage: ./sum num1 num2\n");
        return 1;
    }

    int a = atoi(argv[1]);
    int b = atoi(argv[2]);
    int s = sum(a, b);
    printf("%d + %d = %d\n", a, b, s);

    int vec[] = {1, 2, 3, 4, 5};
    int sum = sum_vec(vec, 5);
    printf("Sum of vector = %d\n", sum);

    return 0;
}