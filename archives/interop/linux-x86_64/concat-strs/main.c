#include <stdio.h>

extern char* concat_strings(char* a, char* b);

int main() {
    char* res = concat_strings("Hello, ", "world!");
    printf("%s\n", res);
    return 0;
}
