#include <stdio.h>

extern int callable_from_c(int);

int main (void) {
    printf("23 %% 3 = %d\n", callable_from_c(23));
    return 0;
}