#ifndef RL_RUNTIME_H
#define RL_RUNTIME_H

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#ifndef M_PI
#define M_PI 3.14159265358979323846
#endif
#ifndef M_E
#define M_E 2.71828182845904523536
#endif
#include <unistd.h>
#include <sys/stat.h>
#include <time.h>
#include <ctype.h>
#include <dirent.h>

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

// ---- map type ----

typedef struct {
    char *key;
    int64_t value;
} rl_map_entry;

typedef struct {
    rl_map_entry *entries;
    uint64_t len;
    uint64_t cap;
} rl_map;

rl_map rl_map_new(void);
void rl_map_set(rl_map *m, const char *key, int64_t val);
int64_t rl_map_get(rl_map m, const char *key);
bool rl_map_contains(rl_map m, const char *key);
uint64_t rl_map_len(rl_map m);
void rl_map_remove(rl_map *m, const char *key);
void rl_print_rl_map(rl_map v);
void rl_println_rl_map(rl_map v);

// ---- set type ----

typedef struct {
    int64_t *data;
    uint64_t len;
    uint64_t cap;
} rl_set;

rl_set rl_set_new(void);
void rl_set_add(rl_set *s, int64_t val);
bool rl_set_contains(rl_set s, int64_t val);
uint64_t rl_set_len(rl_set s);
void rl_set_remove(rl_set *s, int64_t val);
void rl_print_rl_set(rl_set v);
void rl_println_rl_set(rl_set v);

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
void rl_print_rl_array(rl_array v);
void rl_println_rl_array(rl_array v);

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
    rl_array:  rl_print_rl_array, \
    rl_map:   rl_print_rl_map, \
    rl_set:   rl_print_rl_set, \
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
    rl_array:  rl_println_rl_array, \
    rl_map:   rl_println_rl_map, \
    rl_set:   rl_println_rl_set, \
    default:  rl_println_ptr \
)(x)

typedef void rl_never;
rl_never rl_never_fn(void);
#define rl_never() rl_never_fn()

// ---- math ----
int64_t rl_math_factorial(int64_t n);
int64_t rl_math_gcd(int64_t a, int64_t b);
int64_t rl_math_lcm(int64_t a, int64_t b);
bool rl_math_is_prime(int64_t n);
int64_t rl_math_fibonacci(int64_t n);

// ---- time ----
int64_t rl_time_now_ms(void);

// ---- fs ----
int64_t rl_fs_mkdir(rl_string path);

// ---- string ----
rl_string rl_str_to_upper(rl_string s);
rl_string rl_str_to_lower(rl_string s);
rl_string rl_str_trim(rl_string s);
rl_string rl_str_trim_start(rl_string s);
rl_string rl_str_trim_end(rl_string s);
bool rl_str_contains(rl_string haystack, rl_string needle);
bool rl_str_starts_with(rl_string s, rl_string prefix);
bool rl_str_ends_with(rl_string s, rl_string suffix);
rl_string rl_str_replace(rl_string s, rl_string from, rl_string to);
rl_string rl_str_repeat(rl_string s, int64_t count);
int64_t rl_str_index_of(rl_string haystack, rl_string needle);
int64_t rl_str_count(rl_string haystack, rl_string needle);
rl_string rl_str_pad_left(rl_string s, int64_t width, char c);
rl_string rl_str_pad_right(rl_string s, int64_t width, char c);
rl_string rl_str_slice(rl_string s, int64_t start, int64_t end);
rl_string rl_str_reverse(rl_string s);
rl_array rl_str_bytes(rl_string s);
rl_array rl_str_chars(rl_string s);
int64_t rl_str_char_at(rl_string s, int64_t index);
rl_string rl_str_join(rl_array arr, rl_string delim);
rl_array rl_str_split(rl_string s, rl_string delim);

// ---- debug ----
void rl_panic(rl_string msg);
void rl_unreachable(void);
void rl_todo(void);
void rl_assert_fail(rl_string label, int64_t a, int64_t b);
void rl_assert_fail_msg(rl_string label, rl_string msg);
rl_string rl_type_of(int64_t type_tag);
int64_t rl_dbg_int64(int64_t v);
double rl_dbg_float64(double v);
bool rl_dbg_bool(bool v);
rl_string rl_dbg_str(rl_string v);

// ---- path ----
rl_string rl_path_extension(rl_string path);
rl_string rl_path_filename(rl_string path);
rl_string rl_path_parent(rl_string path);
rl_string rl_path_stem(rl_string path);
rl_string rl_path_pop(rl_string path);
rl_string rl_path_join(rl_string path, rl_string target);
rl_string rl_path_push(rl_string path, rl_string target);
rl_string rl_path_set_extension(rl_string path, rl_string ext);
int64_t rl_path_is_dir(rl_string path);
int64_t rl_path_is_file(rl_string path);

// ---- fs ----
int64_t rl_fs_file_size(rl_string path);
int64_t rl_fs_file_modified(rl_string path);
int64_t rl_fs_copy_file(rl_string src, rl_string dst);
int64_t rl_fs_mkdir_all(rl_string path);
int64_t rl_fs_rmdir_all(rl_string path);
rl_array rl_fs_list_dir(rl_string path);
rl_string rl_fs_rename_file(rl_string path, rl_string new_name);

// ---- process ----
rl_string rl_process_cwd(void);
int64_t rl_process_set_cwd(rl_string path);
rl_string rl_process_exec(rl_string cmd);
int64_t rl_process_exec_code(rl_string cmd);
rl_array rl_process_exec_lines(rl_string cmd);
rl_string rl_process_with_exec(rl_string env, rl_string cmd);
int64_t rl_process_with_exec_code(rl_string env, rl_string cmd);
rl_array rl_process_with_exec_lines(rl_string env, rl_string cmd);
rl_array rl_process_args(void);

// ---- time ----
rl_string rl_time_format_time(int64_t timestamp, rl_string pattern);
rl_string rl_time_format_date_str(int64_t timestamp);
rl_string rl_time_format_time_str(int64_t timestamp);
rl_array rl_time_parts(int64_t timestamp);

// ---- io ----
rl_string rl_io_read_file(rl_string path);
rl_array rl_io_read_lines(rl_string path);
rl_string rl_io_read(void);
int64_t rl_io_read_int(void);
double rl_io_read_float(void);
int64_t rl_io_write_file(rl_string path, rl_string content);
int64_t rl_io_append_file(rl_string path, rl_string content);
int64_t rl_io_delete_file(rl_string path);
void rl_io_eprint(rl_string msg);
void rl_io_eprintln(rl_string msg);

// ---- types ----
rl_string rl_types_to_string(int64_t v);
rl_string rl_types_to_bin(int64_t v);
rl_string rl_types_to_hex(int64_t v);
rl_string rl_types_to_oct(int64_t v);

// ---- random ----
int64_t rl_rand_int(void);
double rl_rand_float(void);
bool rl_rand_bool(void);
bool rl_rand_bool_weighted(double weight);
char rl_rand_char(void);
int64_t rl_rand_byte(void);
int64_t rl_rand_int_range(int64_t min, int64_t max);
double rl_rand_float_range(double min, double max);
int64_t rl_rand_dice(int64_t sides);
int64_t rl_rand_range(int64_t stop);
int64_t rl_rand_range_step(int64_t start, int64_t stop, int64_t step);
rl_string rl_rand_string(int64_t count);

// ---- collections (wrappers for rl_string key conversion) ----
int64_t rl_set_add_i64(rl_set *s, int64_t value);
int64_t rl_set_remove_i64(rl_set *s, int64_t value);
int64_t rl_set_contains_i64(rl_set s, int64_t value);
rl_array rl_set_to_array_i64(rl_set s);
int64_t rl_map_contains_s(rl_map m, rl_string key);
int64_t rl_map_remove_s(rl_map *m, rl_string key);
int64_t rl_map_remove_val(rl_map m, rl_string key);
int64_t rl_map_get_s(rl_map m, rl_string key);
rl_array rl_map_keys_s(rl_map m);
rl_array rl_map_values_s(rl_map m);
rl_map rl_map_merge_s(rl_map a, rl_map b);
rl_array rl_map_to_array_s(rl_map m);

// ---- array (extended) ----
rl_array rl_arr_push_i64(rl_array a, int64_t v);
int64_t rl_arr_pop_i64(rl_array a);
rl_array rl_arr_insert_i64(rl_array a, int64_t idx, int64_t v);
rl_array rl_arr_remove_i64(rl_array a, int64_t idx);
rl_array rl_arr_reverse_i64(rl_array a);
rl_array rl_arr_concat_i64(rl_array a, rl_array b);
int64_t rl_arr_first_i64(rl_array a);
int64_t rl_arr_last_i64(rl_array a);
rl_array rl_arr_unique_i64(rl_array a);
rl_array rl_arr_slice_i64(rl_array a, int64_t start, int64_t end);
int64_t rl_arr_contains_i64(rl_array a, int64_t v);
int64_t rl_arr_index_of_i64(rl_array a, int64_t v);
rl_array rl_arr_fill_i64(int64_t v, int64_t count);
rl_array rl_arr_range_i64(int64_t start, int64_t end, int64_t step);
int64_t rl_arr_sum_i64(rl_array a);
int64_t rl_arr_product_i64(rl_array a);
int64_t rl_arr_max_i64(rl_array a);
int64_t rl_arr_min_i64(rl_array a);
rl_array rl_arr_sort_i64(rl_array a);
rl_array rl_arr_flatten_i64(rl_array a);
rl_array rl_arr_push_i64(rl_array a, int64_t v);
int64_t rl_arr_pop_i64(rl_array a);
rl_array rl_arr_insert_i64(rl_array a, int64_t idx, int64_t v);
rl_array rl_arr_remove_i64(rl_array a, int64_t idx);

#endif
