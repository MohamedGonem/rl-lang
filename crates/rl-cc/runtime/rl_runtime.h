#ifndef RL_RUNTIME_H
#define RL_RUNTIME_H

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct {
    const char *data;
    uint64_t len;
    int32_t rc;
} rl_string;

rl_string rl_str_literal(const char *s, uint64_t len);
uint64_t rl_str_len(rl_string s);
rl_string rl_str_concat(rl_string a, rl_string b);
bool rl_str_eq(rl_string a, rl_string b);

// ---- result type ----

typedef struct {
    bool is_ok;
    union {
        int64_t ok_value;
        int64_t err_value;
    } data;
    int32_t err_code;
} rl_result;

rl_result rl_ok(int64_t value);
rl_result rl_err(int64_t value);
rl_result rl_error(int64_t value);

// ---- array type ----

typedef struct {
    void *data;
    uint64_t len;
    uint64_t cap;
    int32_t elem_size;
} rl_array;

rl_array rl_arr_from_vals(const void *vals, uint64_t count, int32_t elem_size);

// ---- print functions ----

void rl_print_int64(int64_t v);
void rl_print_float64(double v);
void rl_print_bool(bool v);
void rl_print_char(char v);
void rl_print_str(rl_string v);
void rl_print_ptr(void *v);

void rl_println_int64(int64_t v);
void rl_println_float64(double v);
void rl_println_bool(bool v);
void rl_println_char(char v);
void rl_println_str(rl_string v);
void rl_println_ptr(void *v);

void rl_print_result(rl_result v);
void rl_println_result(rl_result v);

#define rl_print(x) _Generic((x), \
    int64_t:  rl_print_int64, \
    uint64_t: rl_print_int64, \
    int32_t:  rl_print_int64, \
    uint32_t: rl_print_int64, \
    int16_t:  rl_print_int64, \
    uint16_t: rl_print_int64, \
    int8_t:   rl_print_int64, \
    uint8_t:  rl_print_int64, \
    double:   rl_print_float64, \
    float:    rl_print_float64, \
    bool:     rl_print_bool, \
    char:     rl_print_char, \
    rl_string: rl_print_str, \
    rl_result: rl_print_result, \
    default:  rl_print_ptr \
)(x)

#define rl_println(x) _Generic((x), \
    int64_t:  rl_println_int64, \
    uint64_t: rl_println_int64, \
    int32_t:  rl_println_int64, \
    uint32_t: rl_println_int64, \
    int16_t:  rl_println_int64, \
    uint16_t: rl_println_int64, \
    int8_t:   rl_println_int64, \
    uint8_t:  rl_println_int64, \
    double:   rl_println_float64, \
    float:    rl_println_float64, \
    bool:     rl_println_bool, \
    char:     rl_println_char, \
    rl_string: rl_println_str, \
    rl_result: rl_println_result, \
    default:  rl_println_ptr \
)(x)

typedef void rl_never;
rl_never rl_never_fn(void);
#define rl_never() rl_never_fn()

#endif
