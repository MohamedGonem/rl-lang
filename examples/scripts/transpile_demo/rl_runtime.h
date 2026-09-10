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

// ---- string type ----

typedef struct {
    const char *data;
    uint64_t len;
    int32_t rc;
} rl_string;

rl_string rl_str_literal(const char *s, uint64_t len);
uint64_t rl_str_len(rl_string s);
rl_string rl_str_concat(rl_string a, rl_string b);
bool rl_str_eq(rl_string a, rl_string b);

// ---- forward declarations ----

typedef struct { void *data; uint64_t len; uint64_t cap; int32_t elem_size; int32_t type_tag; } rl_array;
typedef struct rl_closure rl_closure;
typedef struct rl_map rl_map;
typedef struct rl_set rl_set;

// ---- value type (tagged union for map/set storage) ----

enum rl_value_tag {
    RL_VTAG_NULL = 0,
    RL_VTAG_I64,
    RL_VTAG_F64,
    RL_VTAG_BOOL,
    RL_VTAG_CHAR,
    RL_VTAG_STR,
    RL_VTAG_ARR,
    RL_VTAG_MAP,
    RL_VTAG_SET,
    RL_VTAG_CLOSURE,
};

typedef struct rl_value {
    enum rl_value_tag tag;
    union {
        int64_t i64;
        double f64;
        bool boolean;
        rl_string str;
        rl_array arr;
        rl_map *map;
        rl_set *set;
        rl_closure *closure;
    } data;
} rl_value;

typedef struct { char *key; rl_value value; } rl_map_entry;
struct rl_map { rl_map_entry *entries; uint64_t len; uint64_t cap; };
struct rl_set { rl_value *data; uint64_t len; uint64_t cap; };

// ---- result type (tagged union) ----

enum rl_type_tag {
    RL_TAG_NULL = 0,
    RL_TAG_I64,
    RL_TAG_F64,
    RL_TAG_BOOL,
    RL_TAG_CHAR,
    RL_TAG_STR,
    RL_TAG_ARR,
    RL_TAG_MAP,
    RL_TAG_SET,
    RL_TAG_CLOSURE,
};

typedef struct {
    bool is_ok;
    enum rl_type_tag tag;
    union {
        int64_t i64;
        double f64;
        bool boolean;
        rl_string str;
        rl_array arr;
        rl_map map;
        rl_set set;
        rl_closure *closure;
    } data;
    int32_t err_code;
} rl_result;

rl_result rl_ok_null(void);
rl_result rl_ok_i64(int64_t v);
rl_result rl_ok_f64(double v);
rl_result rl_ok_bool(bool v);
rl_result rl_ok_str(rl_string v);
rl_result rl_ok_arr(rl_array v);
rl_result rl_ok_map(rl_map v);
rl_result rl_ok_set(rl_set v);
rl_result rl_err_msg(rl_string msg);
rl_result rl_err_code(int64_t code, rl_string msg);
rl_result rl_err(int64_t v);
rl_result rl_error(int64_t v);

rl_string rl_str_concat_variadic(rl_result *args, uint64_t argc);
rl_result rl_str_format(rl_string tmpl, rl_result *args, uint64_t argc);

// ---- closure type ----

// Closure function pointer type:
//   self  = pointer to the closure itself (for accessing captures)
//   args  = array of rl_result arguments
//   argc  = number of arguments
typedef rl_result (*rl_closure_fn)(rl_closure *self, rl_result *args, uint64_t argc);

struct rl_closure {
    rl_closure_fn fn;
    rl_result *captures;
    uint64_t capture_count;
};

static inline rl_closure rl_closure_new(rl_closure_fn fn, rl_result *captures, uint64_t capture_count) {
    rl_closure c = { .fn = fn, .captures = captures, .capture_count = capture_count };
    return c;
}

static inline rl_result rl_closure_call(rl_closure c, rl_result *args, uint64_t argc) {
    return c.fn(&c, args, argc);
}

static inline rl_result rl_ok_closure(rl_closure v) {
    rl_result r = { .is_ok = true, .tag = RL_TAG_CLOSURE, .err_code = 0 };
    rl_closure *heap = (rl_closure *)malloc(sizeof(rl_closure));
    *heap = v;
    r.data.closure = heap;
    return r;
}

static inline rl_result _rl_identity_result(rl_result v) { return v; }
static inline rl_result _rl_ok_null(void) { return rl_ok_null(); }

// unwrap macros - extract inner value from rl_result
static inline int64_t rl_unwrap_i64(rl_result v) { return v.data.i64; }
static inline double rl_unwrap_f64(rl_result v) { return v.data.f64; }
static inline bool rl_unwrap_bool(rl_result v) { return v.data.boolean; }
static inline rl_string rl_unwrap_str(rl_result v) { return v.data.str; }
static inline rl_array rl_unwrap_arr(rl_result v) { return v.data.arr; }
static inline rl_map rl_unwrap_map(rl_result v) { return v.data.map; }
static inline rl_set rl_unwrap_set(rl_result v) { return v.data.set; }

#define rl_unwrap(x) _Generic((x), \
    rl_result: _rl_unwrap_auto, \
    int64_t:  _rl_id_i64, \
    double:   _rl_id_f64, \
    bool:     _rl_id_bool, \
    rl_string: _rl_id_str, \
    rl_array:  _rl_id_arr, \
    rl_map:    _rl_id_map, \
    rl_set:    _rl_id_set \
)(x)

static inline int64_t _rl_id_i64(int64_t v) { return v; }
static inline double _rl_id_f64(double v) { return v; }
static inline bool _rl_id_bool(bool v) { return v; }
static inline rl_string _rl_id_str(rl_string v) { return v; }
static inline rl_array _rl_id_arr(rl_array v) { return v; }
static inline rl_map _rl_id_map(rl_map v) { return v; }
static inline rl_set _rl_id_set(rl_set v) { return v; }

static inline int64_t _rl_unwrap_auto(rl_result v) { return v.data.i64; }

#define rl_ok(x) _Generic((x), \
    int64_t:  rl_ok_i64, \
    uint64_t: _rl_ok_u64, \
    int32_t:  _rl_ok_i32, \
    uint32_t: _rl_ok_u32, \
    int16_t:  _rl_ok_i16, \
    uint16_t: _rl_ok_u16, \
    int8_t:   _rl_ok_i8, \
    uint8_t:  _rl_ok_u8, \
    char:     _rl_ok_char, \
    double:   rl_ok_f64, \
    float:    _rl_ok_f32, \
    bool:     rl_ok_bool, \
    rl_string: rl_ok_str, \
    rl_array:  rl_ok_arr, \
    rl_map:    rl_ok_map, \
    rl_set:    rl_ok_set, \
    rl_result: _rl_identity_result, \
    void*:    _rl_ok_null \
)(x)

static inline rl_result _rl_ok_u64(uint64_t v) { return rl_ok_i64((int64_t)v); }
static inline rl_result _rl_ok_i32(int32_t v) { return rl_ok_i64(v); }
static inline rl_result _rl_ok_u32(uint32_t v) { return rl_ok_i64(v); }
static inline rl_result _rl_ok_i16(int16_t v) { return rl_ok_i64(v); }
static inline rl_result _rl_ok_u16(uint16_t v) { return rl_ok_i64(v); }
static inline rl_result _rl_ok_i8(int8_t v) { return rl_ok_i64(v); }
static inline rl_result _rl_ok_u8(uint8_t v) { return rl_ok_i64(v); }
static inline rl_result _rl_ok_f32(float v) { return rl_ok_f64(v); }
static inline rl_result _rl_ok_char(char v) {
    rl_result r = { .is_ok = true, .tag = RL_TAG_CHAR, .err_code = 0 };
    r.data.i64 = (int64_t)(unsigned char)v;
    return r;
}

// ---- array type ----

rl_array rl_arr_from_vals(const void *vals, uint64_t count, int32_t elem_size);
rl_array rl_arr_new(int32_t elem_size);

// ---- map type ----

rl_map rl_map_new(void);
void rl_map_set(rl_map *m, const char *key, rl_value val);
rl_value rl_map_get(rl_map m, const char *key);
bool rl_map_contains(rl_map m, const char *key);
uint64_t rl_map_len(rl_map m);
void rl_map_remove(rl_map *m, const char *key);

// ---- set type ----

rl_set rl_set_new(void);
void rl_set_add(rl_set *s, rl_value val);
bool rl_set_contains(rl_set s, rl_value val);
uint64_t rl_set_len(rl_set s);
void rl_set_remove(rl_set *s, rl_value val);

// ---- print functions ----

void rl_print_int64(int64_t v);
void rl_print_float64(double v);
void rl_print_bool(bool v);
void rl_print_char(char v);
void rl_print_str(rl_string v);
void rl_print_ptr(void *v);
void rl_print_null(void);

void rl_println_int64(int64_t v);
void rl_println_float64(double v);
void rl_println_bool(bool v);
void rl_println_char(char v);
void rl_println_str(rl_string v);
void rl_println_ptr(void *v);
void rl_println_null(void);

void rl_print_result(rl_result v);
void rl_println_result(rl_result v);
void rl_print_raw(rl_result v);
void rl_println_raw(rl_result v);
void rl_print_rl_array(rl_array v);
void rl_println_rl_array(rl_array v);
void rl_print_rl_map(rl_map v);
void rl_println_rl_map(rl_map v);
void rl_print_rl_set(rl_set v);
void rl_println_rl_set(rl_set v);
void rl_print_closure(rl_closure v);
void rl_println_closure(rl_closure v);

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
    rl_closure: rl_print_closure, \
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
    rl_closure: rl_println_closure, \
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
rl_result rl_fs_mkdir(rl_string path);

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
char rl_str_char_at(rl_string s, int64_t index);
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
bool rl_path_is_dir(rl_string path);
bool rl_path_is_file(rl_string path);

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
void rl_store_args(int argc, char **argv);

// ---- time ----
rl_string rl_time_format_time(int64_t timestamp, rl_string pattern);
rl_string rl_time_format_date_str(int64_t timestamp);
rl_string rl_time_format_time_str(int64_t timestamp);
rl_array rl_time_parts(int64_t timestamp);

// ---- io ----
rl_result rl_io_read_file(rl_string path);
rl_result rl_io_read_lines(rl_string path);
rl_result rl_io_write_file(rl_string path, rl_string content);
rl_result rl_io_append_file(rl_string path, rl_string content);
rl_string rl_io_read(void);
int64_t rl_io_read_int(void);
double rl_io_read_float(void);
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

// ---- collections (rl_string key wrappers) ----
rl_result rl_set_add_s(rl_set *s, rl_value value);
rl_result rl_set_remove_s(rl_set *s, rl_value value);
rl_result rl_set_contains_s(rl_set s, rl_value value);
rl_array rl_set_to_array(rl_set s);
rl_result rl_map_contains_s(rl_map m, rl_string key);
rl_result rl_map_remove_s(rl_map m, rl_string key);
rl_result rl_map_get_s(rl_map m, rl_string key);
rl_array rl_map_keys_s(rl_map m);
rl_array rl_map_values_s(rl_map m);
rl_map rl_map_merge_s(rl_map a, rl_map b);
rl_array rl_map_to_array_s(rl_map m);

// ---- array (generic) ----
rl_result rl_arr_push(rl_array a, int64_t v);
rl_result rl_arr_pop(rl_array a);
rl_result rl_arr_insert(rl_array a, int64_t idx, int64_t v);
rl_result rl_arr_remove(rl_array a, int64_t idx);
rl_array rl_arr_reverse(rl_array a);
rl_array rl_arr_concat(rl_array a, rl_array b);
rl_result rl_arr_first(rl_array a);
rl_result rl_arr_last(rl_array a);
rl_array rl_arr_unique(rl_array a);
rl_array rl_arr_slice(rl_array a, int64_t start, int64_t end);
rl_result rl_arr_contains(rl_array a, int64_t v);
rl_result rl_arr_index_of(rl_array a, int64_t v);
rl_array rl_arr_fill(int64_t v, int64_t count);
rl_result rl_arr_range(int64_t start, int64_t end, int64_t step);
rl_result rl_arr_sum(rl_array a);
rl_result rl_arr_product(rl_array a);
rl_result rl_arr_max(rl_array a);
rl_result rl_arr_min(rl_array a);
rl_array rl_arr_sort(rl_array a);
rl_array rl_arr_flatten(rl_array a);
rl_result rl_arr_zip(rl_array a, rl_array b);

// ---- closure-consuming array functions ----
rl_result rl_arr_filter_closure(rl_array arr, rl_closure pred);
rl_result rl_arr_map_closure(rl_array arr, rl_closure fn);
rl_result rl_arr_find_closure(rl_array arr, rl_closure pred);
rl_result rl_arr_reduce_closure(rl_array arr, rl_closure fn, rl_result init);
rl_result rl_arr_find_index_closure(rl_array arr, rl_closure pred);
rl_result rl_arr_all_closure(rl_array arr, rl_closure pred);
rl_result rl_arr_any_closure(rl_array arr, rl_closure pred);
rl_result rl_arr_for_each_closure(rl_array arr, rl_closure fn);
rl_result rl_arr_flat_map_closure(rl_array arr, rl_closure fn);
rl_result rl_arr_sort_by_closure(rl_array arr, rl_closure cmp);

// ---- closure-consuming result functions ----
rl_result rl_result_map_closure(rl_result val, rl_closure fn);
rl_result rl_result_map_err_closure(rl_result val, rl_closure fn);

// ---- closure-consuming debug ----
rl_result rl_bench_closure(rl_closure fn, int64_t iterations);

// ---- terminal (ANSI escape codes) ----
rl_result rl_term_enter(void);
rl_result rl_term_leave(void);
rl_result rl_term_clear(void);
rl_result rl_term_clear_line(void);
rl_result rl_term_move(int64_t col, int64_t row);
rl_result rl_term_move_to_col(int64_t col);
rl_result rl_term_move_to_row(int64_t row);
rl_result rl_term_move_up(int64_t n);
rl_result rl_term_move_down(int64_t n);
rl_result rl_term_move_left(int64_t n);
rl_result rl_term_move_right(int64_t n);
rl_result rl_term_next_line(int64_t n);
rl_result rl_term_prev_line(int64_t n);
rl_result rl_term_save_cursor(void);
rl_result rl_term_restore_cursor(void);
rl_result rl_term_hide_cursor(void);
rl_result rl_term_show_cursor(void);
rl_result rl_term_get_size(int64_t *out_cols, int64_t *out_rows);
rl_result rl_term_set_size(int64_t cols, int64_t rows);
rl_result rl_term_set_title(int64_t ch);
rl_result rl_term_scroll_up(int64_t n);
rl_result rl_term_scroll_down(int64_t n);
rl_result rl_term_flush(void);
rl_result rl_term_set_fg(int64_t r, int64_t g, int64_t b);
rl_result rl_term_set_bg(int64_t r, int64_t g, int64_t b);
rl_result rl_term_reset_color(void);
rl_result rl_term_fg(rl_string name);
rl_result rl_term_bg(rl_string name);
rl_result rl_term_bold(void);
rl_result rl_term_dim(void);
rl_result rl_term_italic(void);
rl_result rl_term_underline(void);
rl_result rl_term_blink(void);
rl_result rl_term_reverse(void);
rl_result rl_term_crossed_out(void);
rl_result rl_term_reset_attr(void);
rl_result rl_term_enable_wrap(void);
rl_result rl_term_disable_wrap(void);
rl_result rl_term_begin_sync(void);
rl_result rl_term_end_sync(void);
rl_result rl_term_enable_mouse(void);
rl_result rl_term_disable_mouse(void);
void rl_term_print_inline(rl_result v);
rl_array rl_term_read_key(void);
bool rl_term_poll(int64_t ms);

// ---- result unwrap (with error checking) ----
int64_t rl_result_unwrap_i64(rl_result r);
double rl_result_unwrap_f64(rl_result r);
bool rl_result_unwrap_bool(rl_result r);
rl_string rl_result_unwrap_str(rl_result r);

// ---- math ----
rl_result rl_math_abs(rl_result x);
rl_result rl_math_pow(rl_result base, rl_result exp);

// ---- type checks ----
rl_result rl_is_bool(rl_result x);
rl_result rl_is_int(rl_result x);
rl_result rl_is_float(rl_result x);
rl_result rl_is_string(rl_result x);
rl_result rl_is_null(rl_result x);
rl_result rl_is_char(rl_result x);
rl_result rl_is_byte(rl_result x);
rl_result rl_is_error(rl_result x);

// ---- io ----
rl_result rl_io_read_bytes(rl_string path);

// ---- types ----
rl_result rl_types_error_unwrap(rl_result x);
rl_result rl_types_to_byte(rl_result x);
rl_result rl_types_to_char(rl_result x);

// ---- random (extended) ----
rl_result rl_rand_dices(int64_t count, int64_t sides);
rl_result rl_rand_bytes(int64_t count);
rl_result rl_rand_choice(rl_array arr);
rl_result rl_rand_choices(rl_array arr, int64_t count);
rl_result rl_rand_sample(rl_array arr, int64_t count);
rl_result rl_rand_shuffle(rl_array arr);

#endif
