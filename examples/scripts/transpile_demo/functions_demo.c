#include <stdint.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "rl_runtime.h"

int64_t max(int64_t a, int64_t b) {
    if (a > b) {
        return a;
    } else {
        return b;
    }
}

int64_t my_abs(int64_t n) {
    if (n < (int64_t)0) {
        return -n;
    } else {
        return n;
    }
}

int64_t add(int64_t a, int64_t b) {
    return a + b;
}

int64_t factorial(int64_t n) {
    if (n <= (int64_t)1) {
        return (int64_t)1;
    } else {
        return n * factorial(n - (int64_t)1);
    }
}

int main(int argc, char **argv) {
    int64_t x = (int64_t)10;
    int64_t y = (int64_t)3;
    rl_println(rl_str_literal("=== Comparison operators ===", 28));
    rl_println(max(x, y));
    rl_println(max(y, x));
    rl_println(my_abs((int64_t)-42));
    rl_println(my_abs((int64_t)42));
    rl_println(rl_str_literal("=== Arithmetic chain ===", 24));
    int64_t a = (int64_t)100;
    int64_t b = (int64_t)7;
    rl_println(a + b);
    rl_println(a - b);
    rl_println(a * b);
    rl_println(a / b);
    rl_println(rl_str_literal("=== Boolean logic ===", 21));
    bool p = true;
    bool q = false;
    rl_println(p && q);
    rl_println(p || q);
    rl_println(!q);
    rl_println(rl_str_literal("=== Nested for ===", 18));
    int64_t total = (int64_t)0;
    for (int64_t i = (int64_t)0; i < (int64_t)4; i = i + (int64_t)1) {
        for (int64_t j = (int64_t)0; j < (int64_t)4; j = j + (int64_t)1) {
            total = total + (int64_t)1;
        }
    }
    rl_println(total);
    rl_println(rl_str_literal("=== While countdown ===", 23));
    int64_t count = (int64_t)5;
    while (count > (int64_t)0) {
        rl_println(count);
        count = count - (int64_t)1;
    }
    rl_println(rl_str_literal("=== Functions ===", 17));
    rl_println(add((int64_t)10, (int64_t)20));
    rl_println(factorial((int64_t)6));
    rl_println(rl_str_literal("=== Casts ===", 13));
    double pi = (double)3.14;
    int64_t as_int = (int64_t)pi;
    rl_println(as_int);
    rl_println(rl_str_literal("=== Done ===", 12));
    return 0;
}
