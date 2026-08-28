#include <stdint.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "rl_runtime.h"

int64_t add(int64_t a, int64_t b) {
    return a + b;
}

void greet(rl_string name) {
    rl_println(name);
}

int main(int argc, char **argv) {
    int64_t x = (int64_t)10;
    int64_t y = (int64_t)3;
    int64_t sum = x + y;
    int64_t diff = x - y;
    int64_t prod = x * y;
    int64_t quot = x / y;
    int64_t neg = -x;
    rl_println(rl_str_literal("=== Arithmetic ===", 18));
    rl_println(sum);
    rl_println(diff);
    rl_println(prod);
    rl_println(quot);
    rl_println(neg);
    bool a = true;
    bool b = false;
    bool cmp_gt = x > y;
    bool cmp_lt = x < y;
    bool cmp_eq = x == y;
    bool cmp_ne = x != y;
    bool cmp_le = x <= y;
    bool cmp_ge = x >= y;
    bool and_res = a && b;
    bool or_res = a || b;
    bool not_res = !a;
    rl_println(rl_str_literal("", 0));
    rl_println(rl_str_literal("=== Booleans ===", 16));
    rl_println(a);
    rl_println(b);
    rl_println(cmp_gt);
    rl_println(cmp_lt);
    rl_println(cmp_eq);
    rl_println(cmp_ne);
    rl_println(cmp_le);
    rl_println(cmp_ge);
    rl_println(and_res);
    rl_println(or_res);
    rl_println(not_res);
    double pi = (double)3.14159;
    double half = pi / (double)2;
    rl_println(rl_str_literal("", 0));
    rl_println(rl_str_literal("=== Floats ===", 14));
    rl_println(pi);
    rl_println(half);
    rl_string greeting = rl_str_literal("Hello, world!", 13);
    rl_string lang = rl_str_literal("rl", 2);
    char ch = 'A';
    rl_println(rl_str_literal("", 0));
    rl_println(rl_str_literal("=== Strings ===", 15));
    rl_println(greeting);
    rl_println(lang);
    rl_println(ch);
    rl_println(rl_str_literal("", 0));
    rl_println(rl_str_literal("=== Escapes ===", 15));
    rl_println(rl_str_literal("tab\there", 9));
    rl_println(rl_str_literal("new\nline", 9));
    rl_println(rl_str_literal("back\\slash", 11));
    rl_println(rl_str_literal("quote\"here", 11));
    rl_println(rl_str_literal("single\'quote", 13));
    const int64_t MAX = (int64_t)100;
    const rl_string MSG = rl_str_literal("constant string", 15);
    rl_println(rl_str_literal("", 0));
    rl_println(rl_str_literal("=== Constants ===", 17));
    rl_println(MAX);
    rl_println(MSG);
    int64_t nothing = 0;
    rl_println(rl_str_literal("", 0));
    rl_println(rl_str_literal("=== Null ===", 12));
    rl_println(nothing);
    rl_println(rl_str_literal("", 0));
    rl_println(rl_str_literal("=== Conditionals ===", 20));
    if (x > (int64_t)5) {
        rl_println(rl_str_literal("x is big", 8));
    } else if (x > (int64_t)2) {
        rl_println(rl_str_literal("x is medium", 11));
    } else {
        rl_println(rl_str_literal("x is small", 10));
    }
    rl_println(rl_str_literal("", 0));
    rl_println(rl_str_literal("=== While Loop ===", 18));
    int64_t i = (int64_t)0;
    while (i < (int64_t)3) {
        rl_println(i);
        i = i + (int64_t)1;
    }
    rl_println(rl_str_literal("", 0));
    rl_println(rl_str_literal("=== For Loop ===", 16));
    for (int64_t j = (int64_t)0; j < (int64_t)5; j = j + (int64_t)1) {
        if (j == (int64_t)2) {
            continue;
        }
        if (j == (int64_t)4) {
            break;
        }
        rl_println(j);
    }
    rl_println(rl_str_literal("", 0));
    rl_println(rl_str_literal("=== Functions ===", 17));
    int64_t res = add((int64_t)100, (int64_t)23);
    rl_println(res);
    greet(rl_str_literal("from a function", 15));
    rl_println(rl_str_literal("", 0));
    rl_println(rl_str_literal("=== Casts ===", 13));
    int64_t as_big = (int64_t)42;
    double from_int = (double)as_big;
    rl_println(from_int);
    rl_println(rl_str_literal("", 0));
    rl_println(rl_str_literal("\033[32mAll end-to-end transpiler features demonstrated!\033[0m", 63));
    return 0;
}
