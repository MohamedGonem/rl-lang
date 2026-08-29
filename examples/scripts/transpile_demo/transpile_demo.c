#define _GNU_SOURCE
#define _POSIX_C_SOURCE 200809L
#include <stdint.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "rl_runtime.h"

static rl_result _rl_lambda_0(rl_closure *_self, rl_result *_args, uint64_t _argc) {
    int64_t x = rl_unwrap_i64(_args[0]);
    return rl_ok(x * x);
    return rl_ok_null();
}

static rl_result _rl_lambda_1(rl_closure *_self, rl_result *_args, uint64_t _argc) {
    int64_t x = rl_unwrap_i64(_args[0]);
    int64_t factor = rl_unwrap_i64(_self->captures[0]);
    return rl_ok(x * factor);
    return rl_ok_null();
}

static rl_result _rl_lambda_2(rl_closure *_self, rl_result *_args, uint64_t _argc) {
    int64_t x = rl_unwrap_i64(_args[0]);
    return rl_ok(x * (int64_t)2);
    return rl_ok_null();
}

static rl_result _rl_lambda_3(rl_closure *_self, rl_result *_args, uint64_t _argc) {
    int64_t x = rl_unwrap_i64(_args[0]);
    return rl_ok(x > (int64_t)3);
    return rl_ok_null();
}

static rl_result _rl_lambda_4(rl_closure *_self, rl_result *_args, uint64_t _argc) {
    int64_t x = rl_unwrap_i64(_args[0]);
    return rl_ok(x == (int64_t)3);
    return rl_ok_null();
}

static rl_result _rl_lambda_5(rl_closure *_self, rl_result *_args, uint64_t _argc) {
    int64_t x = rl_unwrap_i64(_args[0]);
    return rl_ok(x > (int64_t)0);
    return rl_ok_null();
}

static rl_result _rl_lambda_6(rl_closure *_self, rl_result *_args, uint64_t _argc) {
    int64_t x = rl_unwrap_i64(_args[0]);
    return rl_ok(x > (int64_t)5);
    return rl_ok_null();
}

static rl_result _rl_lambda_7(rl_closure *_self, rl_result *_args, uint64_t _argc) {
    int64_t x = rl_unwrap_i64(_args[0]);
    rl_println(x);
    return rl_ok_null();
}

static rl_result _rl_lambda_8(rl_closure *_self, rl_result *_args, uint64_t _argc) {
    int64_t a = rl_unwrap_i64(_args[0]);
    int64_t b = rl_unwrap_i64(_args[1]);
    return rl_ok(a - b);
    return rl_ok_null();
}

static rl_result _rl_lambda_9(rl_closure *_self, rl_result *_args, uint64_t _argc) {
    int64_t x = rl_unwrap_i64(_args[0]);
    return rl_ok(rl_arr_from_vals(&(int64_t[]){x, x * (int64_t)10}, 2, (int32_t)sizeof(int64_t)));
    return rl_ok_null();
}



typedef struct { int64_t x; int64_t y; } rl_Record_Point;
void rl_print_rl_Record_Point(rl_Record_Point v) { printf("Record(x: %ld, y: %ld)", (long)v.x, (long)v.y);
}
void rl_println_rl_Record_Point(rl_Record_Point v) { rl_print_rl_Record_Point(v); printf("\n"); }

#define RL_TAG_COLOR_RED ((int64_t)0)
#define RL_TAG_COLOR_GREEN ((int64_t)1)
#define RL_TAG_COLOR_BLUE ((int64_t)2)

typedef struct { int64_t field_0; rl_string field_1; } rl_tuple_2;
void rl_print_rl_tuple_2(rl_tuple_2 v) { printf("(%ld, %.*s)", (long)v.field_0, (int)v.field_1.len, v.field_1.data);
}
void rl_println_rl_tuple_2(rl_tuple_2 v) { rl_print_rl_tuple_2(v); printf("\n"); }
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

int64_t impl_Point_sum(rl_Record_Point a) {
    return a.x + a.y;
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
    rl_println(rl_str_literal("=== ForEach ===", 15));
    rl_array items = rl_arr_from_vals(&(int64_t[]){(int64_t)10, (int64_t)20, (int64_t)30}, 3, (int32_t)sizeof(int64_t));
    rl_array _r_0 = items;
    uint64_t _r_1 = 0;
    for (; _r_1 < _r_0.len; _r_1++) {
        int64_t x = ((int64_t*)_r_0.data)[_r_1];
        rl_println(x);
    }
    rl_println(rl_str_literal("", 0));
    rl_println(rl_str_literal("=== ForRange ===", 16));
    for (int64_t k = 0; k < 5; k++) {
        rl_println(k);
    }
    rl_println(rl_str_literal("", 0));
    rl_println(rl_str_literal("=== Loop ===", 12));
    int64_t counter = (int64_t)0;
    while (1) {
        rl_println(counter);
        counter = counter + (int64_t)1;
        if (counter == (int64_t)3) {
            break;
        }
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
    rl_println(rl_str_literal("=== Tuple Destruction ===", 25));
    rl_tuple_2 pair = (rl_tuple_2){ .field_0 = (int64_t)42, .field_1 = rl_str_literal("hello", 5) };
    rl_tuple_2 _r_2 = pair;
    int64_t px = _r_2.field_0;
    rl_string py = _r_2.field_1;
    rl_println(px);
    rl_println(py);
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
    rl_println(rl_str_literal("=== Impl Methods ===", 20));
    rl_println(impl_Point_sum(p));
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
    rl_println(rl_ok((int64_t)rl_map_len(ages)));
    rl_println(rl_str_literal("", 0));
    rl_println(rl_str_literal("=== Sets ===", 12));
    rl_set s = rl_set_new();
    rl_set_add(&s, (int64_t)1);
    rl_set_add(&s, (int64_t)2);
    rl_set_add(&s, (int64_t)3);
    rl_println(s);
    rl_println(rl_ok((int64_t)rl_set_len(s)));
    rl_println(rl_str_literal("", 0));
    rl_println(rl_str_literal("done", 4));
    rl_println(rl_str_literal("", 0));
    rl_println(rl_str_literal("=== Math Stdlib ===", 19));
    rl_println(rl_math_factorial((int64_t)5));
    rl_println(rl_math_gcd((int64_t)12, (int64_t)8));
    rl_println(rl_math_lcm((int64_t)4, (int64_t)6));
    rl_println(rl_math_is_prime((int64_t)17));
    rl_println(rl_math_fibonacci((int64_t)10));
    rl_println(rl_ok((((int64_t)3) > ((int64_t)7) ? ((int64_t)3) : ((int64_t)7))));
    rl_println(rl_ok((((int64_t)3) < ((int64_t)7) ? ((int64_t)3) : ((int64_t)7))));
    rl_println(rl_ok((int64_t)llabs((int64_t)-42)));
    rl_println(rl_ok(sqrt((double)9)));
    rl_println(sin((double)0));
    rl_println(cos((double)0));
    rl_println(rl_ok(round((double)3.7)));
    rl_println(rl_ok(ceil((double)3.2)));
    rl_println(rl_ok(floor((double)3.8)));
    rl_println(rl_ok(pow((double)2, (double)10)));
    rl_println(rl_ok(log2((double)256)));
    rl_println(rl_str_literal("", 0));
    rl_println(rl_str_literal("=== Math Consts ===", 19));
    rl_println(M_PI);
    rl_println(M_E);
    rl_println((2.0 * M_PI));
    rl_println(((1.0 + sqrt(5.0)) / 2.0));
    rl_println(INFINITY);
    rl_println(NAN);
    rl_println((1.0 / M_PI));
    rl_println((1.0 / sqrt(2.0)));
    rl_println((2.0 / M_PI));
    rl_println((2.0 / sqrt(M_PI)));
    rl_println((M_PI / 2.0));
    rl_println((M_PI / 3.0));
    rl_println((M_PI / 4.0));
    rl_println((M_PI / 6.0));
    rl_println((M_PI / 8.0));
    rl_println(sqrt(2.0));
    rl_println(M_LN2);
    rl_println(M_LN10);
    rl_println(M_LOG2E);
    rl_println((M_LN10 / M_LN2));
    rl_println((M_LN2 / M_LN10));
    rl_println(M_LOG10E);
    rl_println(0.5772156649015329);
    rl_println(rl_str_literal("", 0));
    rl_println(rl_str_literal("=== Bitwise Stdlib ===", 22));
    rl_println(rl_ok((int64_t)255 & (int64_t)15));
    rl_println(rl_ok((int64_t)240 | (int64_t)15));
    rl_println(rl_ok((int64_t)255 ^ (int64_t)15));
    rl_println(rl_ok(~((int64_t)0)));
    rl_println(rl_ok((int64_t)1 << (int64_t)4));
    rl_println(rl_ok((int64_t)128 >> (int64_t)4));
    rl_println(rl_ok(__builtin_popcountll((int64_t)170)));
    rl_println(rl_ok(__builtin_clzll((int64_t)1)));
    rl_println(rl_ok(__builtin_ctzll((int64_t)8)));
    rl_println(rl_str_literal("", 0));
    rl_println(rl_str_literal("=== String Stdlib ===", 21));
    rl_println(rl_str_to_upper(rl_str_literal("hello", 5)));
    rl_println(rl_str_to_lower(rl_str_literal("WORLD", 5)));
    rl_println(rl_str_trim(rl_str_literal("  spaces  ", 10)));
    rl_println(rl_str_contains(rl_str_literal("hello world", 11), rl_str_literal("world", 5)));
    rl_println(rl_str_starts_with(rl_str_literal("hello", 5), rl_str_literal("hel", 3)));
    rl_println(rl_str_ends_with(rl_str_literal("hello", 5), rl_str_literal("llo", 3)));
    rl_println(rl_str_replace(rl_str_literal("foo bar foo", 11), rl_str_literal("foo", 3), rl_str_literal("baz", 3)));
    rl_println(rl_str_repeat(rl_str_literal("ab", 2), (int64_t)3));
    rl_println(rl_str_index_of(rl_str_literal("hello", 5), rl_str_literal("ll", 2)));
    rl_println(rl_str_count(rl_str_literal("anaana", 6), rl_str_literal("ana", 3)));
    rl_println(rl_str_pad_left(rl_str_literal("42", 2), (int64_t)5, '0'));
    rl_println(rl_str_pad_right(rl_str_literal("hi", 2), (int64_t)5, '.'));
    rl_println(rl_ok(rl_str_slice(rl_str_literal("hello", 5), (int64_t)1, (int64_t)4)));
    rl_println(rl_str_reverse(rl_str_literal("abcde", 5)));
    rl_println(rl_ok(rl_str_char_at(rl_str_literal("hello", 5), (int64_t)1)));
    rl_println(rl_str_chars(rl_str_literal("abc", 3)));
    rl_println(rl_str_bytes(rl_str_literal("Hi", 2)));
    rl_println(rl_str_split(rl_str_literal("a,b,c", 5), rl_str_literal(",", 1)));
    rl_println(rl_ok(rl_str_join(rl_arr_from_vals(&(int64_t[]){(int64_t)10, (int64_t)20, (int64_t)30}, 3, (int32_t)sizeof(int64_t)), rl_str_literal("-", 1))));
    rl_println(rl_str_literal("", 0));
    rl_println(rl_str_literal("=== Time / Path ===", 19));
    rl_println(time(NULL));
    rl_println(((bool)(access(rl_str_literal("/tmp", 4).data, F_OK) == 0)));
    rl_println(rl_str_literal("", 0));
    rl_println(rl_str_literal("=== Debug Stdlib ===", 20));
    if (!true) { rl_assert_fail_msg(rl_str_literal("assert", 6), rl_str_literal("", 0)); };
    if (((int64_t)1) != ((int64_t)1)) { rl_assert_fail(rl_str_literal("assert_eq", 8), (int64_t)1, (int64_t)1); };
    if (((int64_t)1) == ((int64_t)2)) { rl_assert_fail(rl_str_literal("assert_ne", 8), (int64_t)1, (int64_t)2); };
    if (((int64_t)1) >= ((int64_t)2)) { rl_assert_fail(rl_str_literal("assert_lt", 8), (int64_t)1, (int64_t)2); };
    if (((int64_t)1) > ((int64_t)1)) { rl_assert_fail(rl_str_literal("assert_le", 8), (int64_t)1, (int64_t)1); };
    if (((int64_t)2) <= ((int64_t)1)) { rl_assert_fail(rl_str_literal("assert_gt", 8), (int64_t)2, (int64_t)1); };
    if (((int64_t)1) < ((int64_t)1)) { rl_assert_fail(rl_str_literal("assert_ge", 8), (int64_t)1, (int64_t)1); };
    if (fabs((double)((double)1) - (double)((double)1)) > 1e-9) { rl_assert_fail(rl_str_literal("assert_approx_eq", 15), (double)1, (double)1); };
    rl_println(rl_type_of(_Generic(((int64_t)42), int64_t: 1, double: 2, bool: 3, rl_string: 4, default: 0)));
    rl_println(rl_type_of(_Generic((rl_str_literal("hi", 2)), int64_t: 1, double: 2, bool: 3, rl_string: 4, default: 0)));
    rl_println(_Generic(((int64_t)99), int64_t: rl_dbg_int64, double: rl_dbg_float64, bool: rl_dbg_bool, rl_string: rl_dbg_str, default: rl_dbg_int64)((int64_t)99));
    rl_println(rl_str_literal("", 0));
    rl_println(rl_str_literal("=== Path Stdlib ===", 19));
    rl_println(rl_path_extension(rl_str_literal("foo.txt", 7)));
    rl_println(rl_path_filename(rl_str_literal("/a/b/c.txt", 10)));
    rl_println(rl_path_parent(rl_str_literal("/a/b/c.txt", 10)));
    rl_println(rl_path_stem(rl_str_literal("/a/b/c.txt", 10)));
    rl_println(rl_path_join(rl_str_literal("/a/b", 4), rl_str_literal("c.txt", 5)));
    rl_println(rl_path_set_extension(rl_str_literal("foo.txt", 7), rl_str_literal("md", 2)));
    rl_println(rl_path_is_dir(rl_str_literal("/tmp", 4)));
    rl_println(rl_path_is_file(rl_str_literal("/etc/hostname", 13)));
    rl_println(rl_str_literal("", 0));
    rl_println(rl_str_literal("=== FS Stdlib ===", 17));
    rl_println(rl_ok(rl_fs_file_size(rl_str_literal("/etc/hostname", 13))));
    rl_println(rl_ok(rl_fs_list_dir(rl_str_literal("/tmp", 4))));
    rl_println(rl_str_literal("", 0));
    rl_println(rl_str_literal("=== Process Stdlib ===", 22));
    rl_println(rl_ok(rl_process_exec(rl_str_literal("echo hello from process", 23))));
    rl_println(rl_ok(rl_process_exec_code(rl_str_literal("true", 4))));
    rl_println(rl_ok(rl_process_exec_lines(rl_str_literal("echo a && echo b", 16))));
    rl_println(rl_ok(rl_process_cwd()));
    rl_println(rl_str_literal("", 0));
    rl_println(rl_str_literal("=== Time Extended ===", 21));
    int64_t ts = time(NULL);
    rl_println(rl_ok(rl_time_format_date_str(ts)));
    rl_println(rl_ok(rl_time_format_time_str(ts)));
    rl_println(rl_ok(rl_time_parts(ts)));
    rl_println(rl_str_literal("", 0));
    rl_println(rl_str_literal("=== IO Extended ===", 19));
    rl_fs_mkdir(rl_str_literal("/tmp/rl_test_io", 15));
    rl_ok(rl_io_write_file(rl_str_literal("/tmp/rl_test_io/test.txt", 24), rl_str_literal("hello world", 11)));
    rl_println(rl_ok(rl_io_read_file(rl_str_literal("/tmp/rl_test_io/test.txt", 24))));
    rl_ok(rl_io_append_file(rl_str_literal("/tmp/rl_test_io/test.txt", 24), rl_str_literal(" appended", 9)));
    rl_println(rl_ok(rl_io_read_file(rl_str_literal("/tmp/rl_test_io/test.txt", 24))));
    rl_println(rl_ok(rl_io_read_lines(rl_str_literal("/tmp/rl_test_io/test.txt", 24))));
    rl_io_eprintln(rl_str_literal("this goes to stderr", 19));
    rl_println(rl_str_literal("", 0));
    rl_println(rl_str_literal("=== Types ===", 13));
    rl_println(rl_ok(rl_types_to_string((int64_t)42)));
    rl_println(rl_ok(rl_types_to_bin((int64_t)255)));
    rl_println(rl_ok(rl_types_to_hex((int64_t)255)));
    rl_println(rl_ok(rl_types_to_oct((int64_t)255)));
    rl_println(rl_ok((int64_t)(double)3.7));
    rl_println(rl_ok((double)(int64_t)42));
    rl_println(rl_ok_bool(((int64_t)1) ? true : false));
    rl_println(rl_str_literal("", 0));
    rl_println(rl_str_literal("=== Random ===", 14));
    rl_println(rl_ok(rl_rand_int() % (int64_t)100));
    rl_println(rl_rand_float());
    rl_println(rl_rand_bool());
    rl_println(rl_ok(rl_rand_int_range((int64_t)1, (int64_t)10)));
    rl_println(rl_ok(rl_rand_dice((int64_t)6)));
    rl_println(rl_ok(rl_rand_range((int64_t)10)));
    rl_println(rl_ok(rl_rand_string((int64_t)5)));
    rl_println(rl_rand_char());
    rl_println(rl_str_literal("", 0));
    rl_println(rl_str_literal("=== Collections Extended ===", 28));
    rl_map ages2 = rl_map_new();
    rl_map_set(&ages2, "alice", (int64_t)30);
    rl_map_set(&ages2, "bob", (int64_t)25);
    rl_println(rl_map_get_s(ages2, rl_str_literal("alice", 5)));
    rl_println(rl_map_contains_s(ages2, rl_str_literal("bob", 3)));
    rl_println(rl_map_contains_s(ages2, rl_str_literal("eve", 3)));
    rl_map_remove_s(ages2, rl_str_literal("bob", 3));
    rl_println(rl_map_contains_s(ages2, rl_str_literal("bob", 3)));
    rl_println(rl_ok(rl_map_to_array_s(ages2)));
    rl_println(rl_str_literal("", 0));
    rl_println(rl_str_literal("=== Array Extended ===", 22));
    rl_array nums2 = rl_arr_from_vals(&(int64_t[]){(int64_t)5, (int64_t)3, (int64_t)1, (int64_t)4, (int64_t)2}, 5, (int32_t)sizeof(int64_t));
    rl_println(rl_arr_first(nums2));
    rl_println(rl_arr_last(nums2));
    rl_println(rl_arr_contains(nums2, (int64_t)3));
    rl_println(rl_arr_contains(nums2, (int64_t)9));
    rl_println(rl_arr_index_of(nums2, (int64_t)4));
    rl_println(rl_arr_sum(nums2));
    rl_println(rl_arr_max(nums2));
    rl_println(rl_arr_min(nums2));
    rl_println(rl_ok(rl_arr_sort(nums2)));
    rl_println(rl_ok(rl_arr_reverse(nums2)));
    rl_println(rl_arr_fill((int64_t)7, (int64_t)3));
    rl_println(rl_arr_range((int64_t)0, (int64_t)5, (int64_t)1));
    rl_println(rl_arr_range((int64_t)10, (int64_t)0, (int64_t)-2));
    rl_println(rl_ok(rl_arr_unique(rl_arr_from_vals(&(int64_t[]){(int64_t)1, (int64_t)2, (int64_t)2, (int64_t)3, (int64_t)3, (int64_t)3}, 6, (int32_t)sizeof(int64_t)))));
    rl_println(rl_ok(rl_arr_concat(rl_arr_from_vals(&(int64_t[]){(int64_t)1, (int64_t)2}, 2, (int32_t)sizeof(int64_t)), rl_arr_from_vals(&(int64_t[]){(int64_t)3, (int64_t)4}, 2, (int32_t)sizeof(int64_t)))));
    rl_println(rl_ok(rl_arr_slice(nums2, (int64_t)1, (int64_t)4)));
    rl_println(rl_arr_push(nums2, (int64_t)99));
    rl_println(rl_str_literal("", 0));
    rl_println(rl_str_literal("=== Closures ===", 16));
    rl_closure square = rl_closure_new(_rl_lambda_0, (rl_result[]){  }, 0);
    rl_println(rl_unwrap_i64(rl_closure_call(square, (rl_result[]){ rl_ok_i64((int64_t)5) }, 1)));
    int64_t factor = (int64_t)3;
    rl_closure triple = rl_closure_new(_rl_lambda_1, (rl_result[]){ rl_ok_i64(factor) }, 1);
    rl_println(rl_unwrap_i64(rl_closure_call(triple, (rl_result[]){ rl_ok_i64((int64_t)4) }, 1)));
    rl_result _r_3 = rl_arr_map_closure(nums, rl_closure_new(_rl_lambda_2, (rl_result[]){  }, 0));
    if (!_r_3.is_ok) {
        rl_println_result(_r_3);
        return 1;
    }
    rl_array doubled = _r_3.data.arr;
    rl_println(doubled);
    rl_result _r_4 = rl_arr_filter_closure(nums, rl_closure_new(_rl_lambda_3, (rl_result[]){  }, 0));
    if (!_r_4.is_ok) {
        rl_println_result(_r_4);
        return 1;
    }
    rl_array large = _r_4.data.arr;
    rl_println(large);
    rl_result _r_5 = rl_arr_find_index_closure(nums, rl_closure_new(_rl_lambda_4, (rl_result[]){  }, 0));
    if (!_r_5.is_ok) {
        rl_println_result(_r_5);
        return 1;
    }
    int64_t idx = _r_5.data.i64;
    rl_println(idx);
    rl_result _r_6 = rl_arr_all_closure(nums, rl_closure_new(_rl_lambda_5, (rl_result[]){  }, 0));
    if (!_r_6.is_ok) {
        rl_println_result(_r_6);
        return 1;
    }
    bool all_pos = _r_6.data.boolean;
    rl_println(all_pos);
    rl_result _r_7 = rl_arr_any_closure(nums, rl_closure_new(_rl_lambda_6, (rl_result[]){  }, 0));
    if (!_r_7.is_ok) {
        rl_println_result(_r_7);
        return 1;
    }
    bool any_big = _r_7.data.boolean;
    rl_println(any_big);
    rl_result _r_8 = rl_arr_for_each_closure(rl_arr_from_vals(&(int64_t[]){(int64_t)10, (int64_t)20, (int64_t)30}, 3, (int32_t)sizeof(int64_t)), rl_closure_new(_rl_lambda_7, (rl_result[]){  }, 0));
    if (!_r_8.is_ok) {
        rl_println_result(_r_8);
        return 1;
    }
    rl_result _r_9 = rl_arr_sort_by_closure(nums, rl_closure_new(_rl_lambda_8, (rl_result[]){  }, 0));
    if (!_r_9.is_ok) {
        rl_println_result(_r_9);
        return 1;
    }
    rl_array sorted = _r_9.data.arr;
    rl_println(sorted);
    rl_array nums3 = rl_arr_from_vals(&(int64_t[]){(int64_t)10, (int64_t)20}, 2, (int32_t)sizeof(int64_t));
    rl_result _r_10 = rl_arr_flat_map_closure(nums, rl_closure_new(_rl_lambda_9, (rl_result[]){  }, 0));
    if (!_r_10.is_ok) {
        rl_println_result(_r_10);
        return 1;
    }
    rl_array flat = _r_10.data.arr;
    rl_println(flat);
    return 0;
}
