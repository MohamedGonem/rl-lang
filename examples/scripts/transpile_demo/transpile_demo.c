#include <stdint.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "rl_runtime.h"

typedef struct { int64_t x; int64_t y; } rl_Record_Point;
void rl_print_rl_Record_Point(rl_Record_Point v) { printf("Record(x: %ld, y: %ld)", (long)v.x, (long)v.y);
}
void rl_println_rl_Record_Point(rl_Record_Point v) { rl_print_rl_Record_Point(v); printf("\n"); }

#define RL_TAG_COLOR_RED ((int64_t)0)
#define RL_TAG_COLOR_GREEN ((int64_t)1)
#define RL_TAG_COLOR_BLUE ((int64_t)2)

typedef struct { int64_t field_0; int64_t field_1; rl_string field_2; } rl_tuple_3;
void rl_print_rl_tuple_3(rl_tuple_3 v) { printf("(%ld, %ld, %.*s)", (long)v.field_0, (long)v.field_1, (int)v.field_2.len, v.field_2.data);
}
void rl_println_rl_tuple_3(rl_tuple_3 v) { rl_print_rl_tuple_3(v); printf("\n"); }

int64_t add(int64_t a, int64_t b) {
    return a + b;
}

void greet(rl_string name) {
    rl_println(name);
}

rl_result safe_div(int64_t a, int64_t b) {
    if (b == (int64_t)0) {
        return rl_err((int64_t)0);
    }
    return rl_ok(a / b);
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
    rl_println(rl_str_literal("=== Tuples ===", 14));
    rl_tuple_3 t = (rl_tuple_3){ .field_0 = (int64_t)1, .field_1 = (int64_t)2, .field_2 = rl_str_literal("three", 5) };
    rl_println_rl_tuple_3(t);
    rl_println(rl_str_literal("", 0));
    rl_println(rl_str_literal("=== Arrays ===", 14));
    rl_array nums = rl_arr_from_vals(&(int64_t[]){(int64_t)10, (int64_t)20, (int64_t)30}, 3, (int32_t)sizeof(int64_t));
    rl_println(((int64_t*)nums.data)[(int64_t)0]);
    ((int64_t*)nums.data)[(int64_t)1] = (int64_t)99;
    rl_println(((int64_t*)nums.data)[(int64_t)1]);
    rl_println(rl_str_literal("", 0));
    rl_println(rl_str_literal("=== Records ===", 15));
    rl_Record_Point p = (rl_Record_Point){ .x = (int64_t)10, .y = (int64_t)20 };
    rl_println(p.x);
    p.x = (int64_t)30;
    rl_println(p.x);
    rl_println(rl_str_literal("", 0));
    rl_println(rl_str_literal("=== Enums ===", 13));
    int64_t /* Color */ c = RL_TAG_COLOR_RED;
    rl_println(c);
    rl_println(rl_str_literal("", 0));
    rl_println(rl_str_literal("=== Match ===", 13));
    if (c == RL_TAG_COLOR_RED) {
        rl_println(rl_str_literal("red", 3));
    }
    else if (c == RL_TAG_COLOR_GREEN) {
        rl_println(rl_str_literal("green", 5));
    }
    else {
        rl_println(rl_str_literal("other", 5));
    }
    rl_println(rl_str_literal("", 0));
    rl_println(rl_str_literal("=== Results ===", 15));
    rl_result r = rl_ok((int64_t)42);
    rl_println(r);
    rl_result r2 = rl_err((int64_t)1);
    rl_println(r2);
    rl_result divided = safe_div((int64_t)10, (int64_t)2);
    rl_println(divided);
    rl_println(rl_str_literal("", 0));
    rl_println(rl_str_literal("=== Maps ===", 12));
    rl_map ages = rl_map_new();
    rl_map_set(&ages, "alice", (int64_t)30);
    rl_map_set(&ages, "bob", (int64_t)25);
    rl_println(ages);
    rl_println(rl_map_len(ages));
    rl_println(rl_str_literal("", 0));
    rl_println(rl_str_literal("=== Sets ===", 12));
    rl_set s = rl_set_new();
    rl_set_add(&s, (int64_t)1);
    rl_set_add(&s, (int64_t)2);
    rl_set_add(&s, (int64_t)3);
    rl_println(s);
    rl_println(rl_set_len(s));
    rl_println(rl_str_literal("", 0));
    rl_println(rl_str_literal("done", 4));
    return 0;
}
