#include <stdint.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "rl_runtime.h"

rl_result divide(int64_t a, int64_t b) {
    if (b == (int64_t)0) {
        return rl_err((int64_t)0);
    } else {
        return rl_ok(a / b);
    }
}

int main(int argc, char **argv) {
    rl_println(rl_str_literal("=== Result type ===", 19));
    rl_result r1 = rl_ok((int64_t)42);
    rl_result r2 = rl_err((int64_t)1);
    rl_println(rl_str_literal("ok(42) created", 14));
    rl_println(rl_str_literal("err(1) created", 14));
    rl_println(rl_str_literal("=== Function returning result ===", 33));
    rl_result r3 = divide((int64_t)10, (int64_t)2);
    rl_println(rl_str_literal("divide(10, 2) created", 21));
    rl_result r4 = divide((int64_t)10, (int64_t)0);
    rl_println(rl_str_literal("divide(10, 0) created", 21));
    rl_println(rl_str_literal("=== Done ===", 12));
    return 0;
}
