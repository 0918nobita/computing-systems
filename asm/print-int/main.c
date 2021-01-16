#include <stdio.h>

extern void printInt(int n);

int main() {
    printInt(10);
    putchar('\n');
    printInt(255);
    return 0;
}
