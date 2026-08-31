#define _GNU_SOURCE
#define _POSIX_C_SOURCE 200809L
#include "rl_runtime.h"

rl_string rl_str_literal(const char *s, uint64_t len) {
    rl_string str = { .data = s, .len = len, .rc = 0 };
    return str;
}

uint64_t rl_str_len(rl_string s) { return s.len; }

rl_string rl_str_concat(rl_string a, rl_string b) {
    uint64_t total = a.len + b.len;
    char *buf = (char *)malloc(total + 1);
    memcpy(buf, a.data, a.len);
    memcpy(buf + a.len, b.data, b.len);
    buf[total] = '\0';
    rl_string result = { .data = buf, .len = total, .rc = 1 };
    return result;
}

bool rl_str_eq(rl_string a, rl_string b) {
    if (a.len != b.len) return false;
    return memcmp(a.data, b.data, a.len) == 0;
}

static void _rl_result_to_str(rl_result v, char **out, uint64_t *out_len) {
    char buf[128];
    int n = 0;
    if (!v.is_ok) { n = snprintf(buf, sizeof(buf), "err(%d)", v.err_code); }
    else switch (v.tag) {
        case RL_TAG_NULL: n = snprintf(buf, sizeof(buf), "null"); break;
        case RL_TAG_I64: n = snprintf(buf, sizeof(buf), "%ld", (long)v.data.i64); break;
        case RL_TAG_F64: n = snprintf(buf, sizeof(buf), "%g", v.data.f64); break;
        case RL_TAG_BOOL: n = snprintf(buf, sizeof(buf), "%s", v.data.boolean ? "true" : "false"); break;
        case RL_TAG_CHAR: n = snprintf(buf, sizeof(buf), "%c", (char)(unsigned char)v.data.i64); break;
        case RL_TAG_STR: { char *d = malloc(v.data.str.len + 1); memcpy(d, v.data.str.data, v.data.str.len); d[v.data.str.len] = '\0'; *out = d; *out_len = v.data.str.len; return; }
        default: n = snprintf(buf, sizeof(buf), "<value>"); break;
    }
    char *dup = malloc(n + 1);
    memcpy(dup, buf, n + 1);
    *out = dup;
    *out_len = n;
}

rl_string rl_str_concat_variadic(rl_result *args, uint64_t argc) {
    uint64_t total = 0;
    char **parts = malloc(argc * sizeof(char *));
    uint64_t *part_lens = malloc(argc * sizeof(uint64_t));
    for (uint64_t i = 0; i < argc; i++) {
        _rl_result_to_str(args[i], &parts[i], &part_lens[i]);
        total += part_lens[i];
    }
    char *buf = malloc(total + 1);
    uint64_t pos = 0;
    for (uint64_t i = 0; i < argc; i++) {
        memcpy(buf + pos, parts[i], part_lens[i]);
        pos += part_lens[i];
        if (parts[i] != args[i].data.str.data) free(parts[i]);
    }
    buf[total] = '\0';
    free(parts);
    free(part_lens);
    return (rl_string){ .data = buf, .len = total, .rc = 1 };
}

rl_result rl_str_format(rl_string tmpl, rl_result *args, uint64_t argc) {
    uint64_t total = 0;
    char **parts = malloc((argc + 1) * sizeof(char *));
    uint64_t *part_lens = malloc((argc + 1) * sizeof(uint64_t));
    uint64_t part_count = 0;
    uint64_t arg_idx = 0;
    uint64_t i = 0;
    while (i < tmpl.len) {
        if (tmpl.data[i] == '{' && i + 1 < tmpl.len && tmpl.data[i + 1] == '}') {
            if (arg_idx >= argc) {
                for (uint64_t j = 0; j < part_count; j++) if (parts[j] != tmpl.data + 0) free(parts[j]);
                free(parts); free(part_lens);
                return rl_err_msg(rl_str_literal("format: not enough arguments for placeholders", 47));
            }
            _rl_result_to_str(args[arg_idx], &parts[part_count], &part_lens[part_count]);
            total += part_lens[part_count];
            part_count++;
            arg_idx++;
            i += 2;
        } else {
            uint64_t start = i;
            while (i < tmpl.len && !(tmpl.data[i] == '{' && i + 1 < tmpl.len && tmpl.data[i + 1] == '}')) i++;
            uint64_t seg_len = i - start;
            parts[part_count] = malloc(seg_len + 1);
            memcpy(parts[part_count], tmpl.data + start, seg_len);
            parts[part_count][seg_len] = '\0';
            part_lens[part_count] = seg_len;
            total += seg_len;
            part_count++;
        }
    }
    if (arg_idx < argc) {
        for (uint64_t j = 0; j < part_count; j++) if (parts[j] != tmpl.data + 0) free(parts[j]);
        free(parts); free(part_lens);
        return rl_err_msg(rl_str_literal("format: too many arguments for placeholders", 42));
    }
    char *buf = malloc(total + 1);
    uint64_t pos = 0;
    for (uint64_t j = 0; j < part_count; j++) {
        memcpy(buf + pos, parts[j], part_lens[j]);
        pos += part_lens[j];
        free(parts[j]);
    }
    buf[total] = '\0';
    free(parts);
    free(part_lens);
    return rl_ok_str((rl_string){ .data = buf, .len = total, .rc = 1 });
}

rl_result rl_arr_zip(rl_array a, rl_array b) {
    uint64_t min_len = a.len < b.len ? a.len : b.len;
    int64_t *a_elems = (int64_t *)a.data;
    int64_t *b_elems = (int64_t *)b.data;
    uint64_t cap = 16;
    int64_t *buf = malloc(cap * sizeof(int64_t));
    uint64_t count = 0;
    for (uint64_t i = 0; i < min_len; i++) {
        if (count + 2 > cap) { cap *= 2; buf = realloc(buf, cap * sizeof(int64_t)); }
        buf[count++] = a_elems[i];
        buf[count++] = b_elems[i];
    }
    return rl_ok_arr(rl_arr_from_vals(buf, count, sizeof(int64_t)));
}

// ---- result type ----

rl_result rl_ok_null(void) {
    rl_result r = { .is_ok = true, .tag = RL_TAG_NULL, .err_code = 0 };
    return r;
}

rl_result rl_ok_i64(int64_t v) {
    rl_result r = { .is_ok = true, .tag = RL_TAG_I64, .data.i64 = v, .err_code = 0 };
    return r;
}

rl_result rl_ok_f64(double v) {
    rl_result r = { .is_ok = true, .tag = RL_TAG_F64, .data.f64 = v, .err_code = 0 };
    return r;
}

rl_result rl_ok_bool(bool v) {
    rl_result r = { .is_ok = true, .tag = RL_TAG_BOOL, .data.boolean = v, .err_code = 0 };
    return r;
}

rl_result rl_ok_str(rl_string v) {
    rl_result r = { .is_ok = true, .tag = RL_TAG_STR, .data.str = v, .err_code = 0 };
    return r;
}

rl_result rl_ok_arr(rl_array v) {
    rl_result r = { .is_ok = true, .tag = RL_TAG_ARR, .data.arr = v, .err_code = 0 };
    return r;
}

rl_result rl_ok_map(rl_map v) {
    rl_result r = { .is_ok = true, .tag = RL_TAG_MAP, .data.map = v, .err_code = 0 };
    return r;
}

rl_result rl_ok_set(rl_set v) {
    rl_result r = { .is_ok = true, .tag = RL_TAG_SET, .data.set = v, .err_code = 0 };
    return r;
}

static rl_result rl_make_err(int64_t code, const char *msg) {
    rl_string s = rl_str_literal(msg, strlen(msg));
    rl_result r = { .is_ok = false, .tag = RL_TAG_STR, .data.str = s, .err_code = (int32_t)code };
    return r;
}

rl_result rl_err_msg(rl_string msg) {
    rl_result r = { .is_ok = false, .tag = RL_TAG_STR, .data.str = msg, .err_code = 0 };
    return r;
}

rl_result rl_err_code(int64_t code, rl_string msg) {
    rl_result r = { .is_ok = false, .tag = RL_TAG_STR, .data.str = msg, .err_code = (int32_t)code };
    return r;
}

rl_result rl_err(int64_t v) {
    rl_result r = { .is_ok = false, .tag = RL_TAG_I64, .data.i64 = v, .err_code = 0 };
    return r;
}

rl_result rl_error(int64_t v) {
    rl_result r = { .is_ok = false, .tag = RL_TAG_I64, .data.i64 = v, .err_code = -1 };
    return r;
}

// ---- array type ----

rl_array rl_arr_from_vals(const void *vals, uint64_t count, int32_t elem_size) {
    rl_array arr;
    arr.len = count;
    arr.cap = count;
    arr.elem_size = elem_size;
    arr.type_tag = RL_TAG_I64;
    if (count == 0) {
        arr.data = NULL;
    } else {
        arr.data = malloc(count * elem_size);
        memcpy(arr.data, vals, count * elem_size);
    }
    return arr;
}

rl_array rl_arr_new(int32_t elem_size) {
    rl_array arr = { .data = NULL, .len = 0, .cap = 0, .elem_size = elem_size, .type_tag = RL_TAG_I64 };
    return arr;
}

// ---- map type ----

rl_map rl_map_new(void) {
    rl_map m = { .entries = NULL, .len = 0, .cap = 0 };
    return m;
}

static void rl_map_grow(rl_map *m, uint64_t needed) {
    if (m->cap >= needed) return;
    uint64_t new_cap = m->cap == 0 ? 8 : m->cap * 2;
    while (new_cap < needed) new_cap *= 2;
    m->entries = realloc(m->entries, new_cap * sizeof(rl_map_entry));
    m->cap = new_cap;
}

void rl_map_set(rl_map *m, const char *key, rl_value val) {
    for (uint64_t i = 0; i < m->len; i++) {
        if (strcmp(m->entries[i].key, key) == 0) {
            m->entries[i].value = val;
            return;
        }
    }
    rl_map_grow(m, m->len + 1);
    m->entries[m->len].key = strdup(key);
    m->entries[m->len].value = val;
    m->len++;
}

rl_value rl_map_get(rl_map m, const char *key) {
    for (uint64_t i = 0; i < m.len; i++) {
        if (strcmp(m.entries[i].key, key) == 0) {
            return m.entries[i].value;
        }
    }
    rl_value null_val = { .tag = RL_VTAG_NULL, .data.i64 = 0 };
    return null_val;
}

bool rl_map_contains(rl_map m, const char *key) {
    for (uint64_t i = 0; i < m.len; i++) {
        if (strcmp(m.entries[i].key, key) == 0) return true;
    }
    return false;
}

uint64_t rl_map_len(rl_map m) { return m.len; }

void rl_map_remove(rl_map *m, const char *key) {
    for (uint64_t i = 0; i < m->len; i++) {
        if (strcmp(m->entries[i].key, key) == 0) {
            free(m->entries[i].key);
            m->entries[i] = m->entries[m->len - 1];
            m->len--;
            return;
        }
    }
}

// ---- set type ----

rl_set rl_set_new(void) {
    rl_set s = { .data = NULL, .len = 0, .cap = 0 };
    return s;
}

static void rl_set_grow(rl_set *s, uint64_t needed) {
    if (s->cap >= needed) return;
    uint64_t new_cap = s->cap == 0 ? 8 : s->cap * 2;
    while (new_cap < needed) new_cap *= 2;
    s->data = realloc(s->data, new_cap * sizeof(rl_value));
    s->cap = new_cap;
}

static bool rl_value_eq(rl_value a, rl_value b) {
    if (a.tag != b.tag) return false;
    switch (a.tag) {
        case RL_VTAG_NULL: return true;
        case RL_VTAG_I64: return a.data.i64 == b.data.i64;
        case RL_VTAG_F64: return a.data.f64 == b.data.f64;
        case RL_VTAG_BOOL: return a.data.boolean == b.data.boolean;
        case RL_VTAG_CHAR: return a.data.i64 == b.data.i64;
        case RL_VTAG_STR: return rl_str_eq(a.data.str, b.data.str);
        default: return false;
    }
}

void rl_set_add(rl_set *s, rl_value val) {
    for (uint64_t i = 0; i < s->len; i++) {
        if (rl_value_eq(s->data[i], val)) return;
    }
    rl_set_grow(s, s->len + 1);
    s->data[s->len++] = val;
}

bool rl_set_contains(rl_set s, rl_value val) {
    for (uint64_t i = 0; i < s.len; i++) {
        if (rl_value_eq(s.data[i], val)) return true;
    }
    return false;
}

uint64_t rl_set_len(rl_set s) { return s.len; }

void rl_set_remove(rl_set *s, rl_value val) {
    for (uint64_t i = 0; i < s->len; i++) {
        if (rl_value_eq(s->data[i], val)) {
            s->data[i] = s->data[s->len - 1];
            s->len--;
            return;
        }
    }
}

// ---- print functions ----

static void rl_print_f64(double v) {
    if (isnan(v)) { printf("NaN"); return; }
    if (isinf(v) > 0) { printf("inf"); return; }
    if (isinf(v) < 0) { printf("-inf"); return; }
    char buf[64];
    int best_prec = 17;
    for (int prec = 1; prec <= 17; prec++) {
        char tmp[64];
        snprintf(tmp, sizeof(tmp), "%.*g", prec, v);
        if (strtod(tmp, NULL) == v) {
            best_prec = prec;
            break;
        }
    }
    snprintf(buf, sizeof(buf), "%.*g", best_prec, v);
    printf("%s", buf);
}

static void rl_print_i64_val(int64_t v) { printf("%ld", v); }
static void rl_print_f64_val(double v) { rl_print_f64(v); }
static void rl_print_bool_val(bool v) { printf(v ? "true" : "false"); }
static void rl_print_str_val(rl_string v) { printf("%.*s", (int)v.len, v.data); }
static void rl_print_arr_val(rl_array v) { rl_print_rl_array(v); }
static void rl_print_map_val(rl_map v) { rl_print_rl_map(v); }
static void rl_print_set_val(rl_set v) { rl_print_rl_set(v); }

void rl_print_int64(int64_t v) { printf("%ld", v); }
void rl_print_float64(double v) { rl_print_f64(v); }
void rl_print_bool(bool v) { printf(v ? "true" : "false"); }
void rl_print_char(char v) { printf("%c", v); }
void rl_print_str(rl_string v) { printf("%.*s", (int)v.len, v.data); }
void rl_print_ptr(void *v) { printf("<ptr:%p>", v); }
void rl_print_null(void) { printf("null"); }

void rl_println_int64(int64_t v) { printf("%ld\n", v); }
void rl_println_float64(double v) { rl_print_f64(v); printf("\n"); }
void rl_println_bool(bool v) { printf("%s\n", v ? "true" : "false"); }
void rl_println_char(char v) { printf("%c\n", v); }
void rl_println_str(rl_string v) { printf("%.*s\n", (int)v.len, v.data); }
void rl_println_ptr(void *v) { printf("<ptr:%p>\n", v); }
void rl_println_null(void) { printf("null\n"); }

static void rl_print_result_inner(rl_result v) {
    if (v.is_ok) {
        printf("ok(");
        switch (v.tag) {
             case RL_TAG_NULL: printf("null"); break;
            case RL_TAG_I64: rl_print_i64_val(v.data.i64); break;
            case RL_TAG_F64: rl_print_f64_val(v.data.f64); break;
            case RL_TAG_BOOL: rl_print_bool_val(v.data.boolean); break;
            case RL_TAG_CHAR: printf("%c", (char)(unsigned char)v.data.i64); break;
            case RL_TAG_STR: printf("%.*s", (int)v.data.str.len, v.data.str.data); break;
            case RL_TAG_ARR: rl_print_arr_val(v.data.arr); break;
            case RL_TAG_MAP: rl_print_map_val(v.data.map); break;
            case RL_TAG_SET: rl_print_set_val(v.data.set); break;
            case RL_TAG_CLOSURE: printf("<fn>"); break;
        }
        printf(")");
    } else {
        printf("err(");
        switch (v.tag) {
            case RL_TAG_NULL: printf("null"); break;
            case RL_TAG_I64: rl_print_i64_val(v.data.i64); break;
            case RL_TAG_F64: rl_print_f64_val(v.data.f64); break;
            case RL_TAG_BOOL: rl_print_bool_val(v.data.boolean); break;
            case RL_TAG_CHAR: printf("%c", (char)(unsigned char)v.data.i64); break;
            case RL_TAG_STR: printf("%.*s", (int)v.data.str.len, v.data.str.data); break;
            case RL_TAG_ARR: rl_print_arr_val(v.data.arr); break;
            case RL_TAG_MAP: rl_print_map_val(v.data.map); break;
            case RL_TAG_SET: rl_print_set_val(v.data.set); break;
            case RL_TAG_CLOSURE: printf("<fn>"); break;
        }
        printf(")");
    }
}

void rl_print_result(rl_result v) { rl_print_result_inner(v); }
void rl_println_result(rl_result v) { rl_print_result_inner(v); printf("\n"); }

void rl_print_rl_array(rl_array v) {
    printf("[");
    for (uint64_t i = 0; i < v.len; i++) {
        if (i > 0) printf(", ");
        if (v.type_tag == RL_TAG_CHAR) {
            printf("%c", ((char *)v.data)[i]);
        } else if (v.elem_size == sizeof(int64_t)) {
            printf("%ld", (long)((int64_t *)v.data)[i]);
        } else if (v.elem_size == sizeof(double)) {
            { double fv = ((double *)v.data)[i]; rl_print_f64(fv); }
        } else if (v.elem_size == sizeof(rl_string)) {
            rl_string s = ((rl_string *)v.data)[i];
            printf("%.*s", (int)s.len, s.data);
        } else if (v.elem_size == sizeof(bool)) {
            printf("%s", ((bool *)v.data)[i] ? "true" : "false");
        } else {
            printf("...");
        }
    }
    printf("]");
}

void rl_println_rl_array(rl_array v) {
    rl_print_rl_array(v);
    printf("\n");
}

static void _rl_print_value(rl_value v) {
    switch (v.tag) {
        case RL_VTAG_NULL: printf("null"); break;
        case RL_VTAG_I64: printf("%ld", (long)v.data.i64); break;
        case RL_VTAG_F64: printf("%g", v.data.f64); break;
        case RL_VTAG_BOOL: printf("%s", v.data.boolean ? "true" : "false"); break;
        case RL_VTAG_CHAR: printf("%c", (char)(unsigned char)v.data.i64); break;
        case RL_VTAG_STR: printf("%.*s", (int)v.data.str.len, v.data.str.data); break;
        case RL_VTAG_ARR: rl_print_rl_array(v.data.arr); break;
        case RL_VTAG_CLOSURE: printf("<fn>"); break;
        default: printf("<value>"); break;
    }
}

void rl_print_rl_map(rl_map v) {
    printf("{");
    for (uint64_t i = 0; i < v.len; i++) {
        if (i > 0) printf(", ");
        printf("%s: ", v.entries[i].key);
        _rl_print_value(v.entries[i].value);
    }
    printf("}");
}

void rl_println_rl_map(rl_map v) {
    rl_print_rl_map(v);
    printf("\n");
}

void rl_print_rl_set(rl_set v) {
    printf("{");
    for (uint64_t i = 0; i < v.len; i++) {
        if (i > 0) printf(", ");
        _rl_print_value(v.data[i]);
    }
    printf("}");
}

void rl_println_rl_set(rl_set v) {
    rl_print_rl_set(v);
    printf("\n");
}

void rl_print_closure(rl_closure v) {
    printf("<fn>");
}

void rl_println_closure(rl_closure v) {
    rl_print_closure(v);
    printf("\n");
}

// ---- math ----

int64_t rl_math_factorial(int64_t n) {
    if (n < 0) return 0;
    int64_t result = 1;
    for (int64_t i = 2; i <= n; i++) { result *= i; }
    return result;
}

int64_t rl_math_gcd(int64_t a, int64_t b) {
    a = a < 0 ? -a : a;
    b = b < 0 ? -b : b;
    while (b) { int64_t t = b; b = a % b; a = t; }
    return a;
}

int64_t rl_math_lcm(int64_t a, int64_t b) {
    if (a == 0 || b == 0) return 0;
    a = a < 0 ? -a : a;
    b = b < 0 ? -b : b;
    return (a / rl_math_gcd(a, b)) * b;
}

bool rl_math_is_prime(int64_t n) {
    if (n < 2) return false;
    if (n == 2) return true;
    if (n % 2 == 0) return false;
    for (int64_t i = 3; i * i <= n; i += 2) {
        if (n % i == 0) return false;
    }
    return true;
}

int64_t rl_math_fibonacci(int64_t n) {
    if (n <= 0) return 0;
    if (n == 1) return 1;
    int64_t a = 0, b = 1;
    for (int64_t i = 2; i <= n; i++) {
        int64_t t = a + b;
        a = b;
        b = t;
    }
    return b;
}

// ---- time ----

int64_t rl_time_now_ms(void) {
    struct timespec ts;
    clock_gettime(CLOCK_REALTIME, &ts);
    return (int64_t)ts.tv_sec * 1000 + ts.tv_nsec / 1000000;
}

// ---- fs ----

rl_result rl_fs_mkdir(rl_string path) {
    char buf[path.len + 1];
    memcpy(buf, path.data, path.len);
    buf[path.len] = '\0';
    int rc = mkdir(buf, 0755);
    if (rc == 0) return rl_ok_i64(0);
    return rl_make_err(-1, "mkdir failed");
}

// ---- string ----

rl_string rl_str_to_upper(rl_string s) {
    char *buf = malloc(s.len + 1);
    for (uint64_t i = 0; i < s.len; i++) {
        buf[i] = toupper((unsigned char)s.data[i]);
    }
    buf[s.len] = '\0';
    rl_string result = { .data = buf, .len = s.len, .rc = 1 };
    return result;
}

rl_string rl_str_to_lower(rl_string s) {
    char *buf = malloc(s.len + 1);
    for (uint64_t i = 0; i < s.len; i++) {
        buf[i] = tolower((unsigned char)s.data[i]);
    }
    buf[s.len] = '\0';
    rl_string result = { .data = buf, .len = s.len, .rc = 1 };
    return result;
}

static uint64_t rl_str_skip_space_start(const char *s, uint64_t len) {
    uint64_t i = 0;
    while (i < len && (s[i] == ' ' || s[i] == '\t' || s[i] == '\n' || s[i] == '\r')) i++;
    return i;
}

static uint64_t rl_str_skip_space_end(const char *s, uint64_t len) {
    uint64_t i = len;
    while (i > 0 && (s[i-1] == ' ' || s[i-1] == '\t' || s[i-1] == '\n' || s[i-1] == '\r')) i--;
    return i;
}

rl_string rl_str_trim(rl_string s) {
    uint64_t start = rl_str_skip_space_start(s.data, s.len);
    uint64_t end = rl_str_skip_space_end(s.data, s.len);
    if (start >= end) {
        rl_string result = { .data = "", .len = 0, .rc = 1 };
        return result;
    }
    char *buf = malloc(end - start + 1);
    memcpy(buf, s.data + start, end - start);
    buf[end - start] = '\0';
    rl_string result = { .data = buf, .len = end - start, .rc = 1 };
    return result;
}

rl_string rl_str_trim_start(rl_string s) {
    uint64_t start = rl_str_skip_space_start(s.data, s.len);
    char *buf = malloc(s.len - start + 1);
    memcpy(buf, s.data + start, s.len - start);
    buf[s.len - start] = '\0';
    rl_string result = { .data = buf, .len = s.len - start, .rc = 1 };
    return result;
}

rl_string rl_str_trim_end(rl_string s) {
    uint64_t end = rl_str_skip_space_end(s.data, s.len);
    char *buf = malloc(end + 1);
    memcpy(buf, s.data, end);
    buf[end] = '\0';
    rl_string result = { .data = buf, .len = end, .rc = 1 };
    return result;
}

bool rl_str_contains(rl_string haystack, rl_string needle) {
    if (needle.len == 0) return true;
    if (needle.len > haystack.len) return false;
    for (uint64_t i = 0; i <= haystack.len - needle.len; i++) {
        if (memcmp(haystack.data + i, needle.data, needle.len) == 0) return true;
    }
    return false;
}

bool rl_str_starts_with(rl_string s, rl_string prefix) {
    if (prefix.len > s.len) return false;
    return memcmp(s.data, prefix.data, prefix.len) == 0;
}

bool rl_str_ends_with(rl_string s, rl_string suffix) {
    if (suffix.len > s.len) return false;
    return memcmp(s.data + s.len - suffix.len, suffix.data, suffix.len) == 0;
}

rl_string rl_str_replace(rl_string s, rl_string from, rl_string to) {
    if (from.len == 0) {
        char *buf = malloc(s.len + 1);
        memcpy(buf, s.data, s.len);
        buf[s.len] = '\0';
        rl_string result = { .data = buf, .len = s.len, .rc = 1 };
        return result;
    }
    uint64_t count = 0;
    uint64_t pos = 0;
    while (pos <= s.len) {
        if (pos + from.len <= s.len && memcmp(s.data + pos, from.data, from.len) == 0) {
            count++;
            pos += from.len;
        } else {
            pos++;
        }
    }
    if (count == 0) {
        char *buf = malloc(s.len + 1);
        memcpy(buf, s.data, s.len);
        buf[s.len] = '\0';
        rl_string result = { .data = buf, .len = s.len, .rc = 1 };
        return result;
    }
    uint64_t new_len = s.len - count * from.len + count * to.len;
    char *buf = malloc(new_len + 1);
    uint64_t w = 0;
    pos = 0;
    while (pos < s.len) {
        if (pos + from.len <= s.len && memcmp(s.data + pos, from.data, from.len) == 0) {
            memcpy(buf + w, to.data, to.len);
            w += to.len;
            pos += from.len;
        } else {
            buf[w++] = s.data[pos++];
        }
    }
    buf[new_len] = '\0';
    rl_string result = { .data = buf, .len = new_len, .rc = 1 };
    return result;
}

rl_string rl_str_repeat(rl_string s, int64_t count) {
    if (count <= 0 || s.len == 0) {
        rl_string result = { .data = "", .len = 0, .rc = 1 };
        return result;
    }
    uint64_t total = s.len * (uint64_t)count;
    char *buf = malloc(total + 1);
    for (int64_t i = 0; i < count; i++) {
        memcpy(buf + i * s.len, s.data, s.len);
    }
    buf[total] = '\0';
    rl_string result = { .data = buf, .len = total, .rc = 1 };
    return result;
}

int64_t rl_str_index_of(rl_string haystack, rl_string needle) {
    if (needle.len == 0) return 0;
    if (needle.len > haystack.len) return -1;
    for (uint64_t i = 0; i <= haystack.len - needle.len; i++) {
        if (memcmp(haystack.data + i, needle.data, needle.len) == 0) return (int64_t)i;
    }
    return -1;
}

int64_t rl_str_count(rl_string haystack, rl_string needle) {
    if (needle.len == 0) return 0;
    int64_t count = 0;
    uint64_t pos = 0;
    while (pos <= haystack.len) {
        if (pos + needle.len <= haystack.len && memcmp(haystack.data + pos, needle.data, needle.len) == 0) {
            count++;
            pos += needle.len;
        } else {
            pos++;
        }
    }
    return count;
}

rl_string rl_str_pad_left(rl_string s, int64_t width, char c) {
    int64_t pad = width - (int64_t)s.len;
    if (pad <= 0) {
        char *buf = malloc(s.len + 1);
        memcpy(buf, s.data, s.len);
        buf[s.len] = '\0';
        rl_string result = { .data = buf, .len = s.len, .rc = 1 };
        return result;
    }
    uint64_t total = (uint64_t)pad + s.len;
    char *buf = malloc(total + 1);
    for (int64_t i = 0; i < pad; i++) buf[i] = c;
    memcpy(buf + pad, s.data, s.len);
    buf[total] = '\0';
    rl_string result = { .data = buf, .len = total, .rc = 1 };
    return result;
}

rl_string rl_str_pad_right(rl_string s, int64_t width, char c) {
    int64_t pad = width - (int64_t)s.len;
    if (pad <= 0) {
        char *buf = malloc(s.len + 1);
        memcpy(buf, s.data, s.len);
        buf[s.len] = '\0';
        rl_string result = { .data = buf, .len = s.len, .rc = 1 };
        return result;
    }
    uint64_t total = s.len + (uint64_t)pad;
    char *buf = malloc(total + 1);
    memcpy(buf, s.data, s.len);
    for (int64_t i = 0; i < pad; i++) buf[s.len + i] = c;
    buf[total] = '\0';
    rl_string result = { .data = buf, .len = total, .rc = 1 };
    return result;
}

rl_string rl_str_slice(rl_string s, int64_t start, int64_t end) {
    if (start < 0) start = 0;
    if (end > (int64_t)s.len) end = (int64_t)s.len;
    if (start >= end) {
        rl_string result = { .data = "", .len = 0, .rc = 1 };
        return result;
    }
    uint64_t len = (uint64_t)(end - start);
    char *buf = malloc(len + 1);
    memcpy(buf, s.data + start, len);
    buf[len] = '\0';
    rl_string result = { .data = buf, .len = len, .rc = 1 };
    return result;
}

rl_string rl_str_reverse(rl_string s) {
    char *buf = malloc(s.len + 1);
    uint64_t j = 0;
    uint64_t i = s.len;
    while (i > 0) {
        unsigned char c = (unsigned char)s.data[i - 1];
        uint64_t char_len;
        if (c < 0x80) char_len = 1;
        else if (c < 0xE0) char_len = 2;
        else if (c < 0xF0) char_len = 3;
        else char_len = 4;
        i -= char_len;
        memcpy(buf + j, s.data + i, char_len);
        j += char_len;
    }
    buf[s.len] = '\0';
    rl_string result = { .data = buf, .len = s.len, .rc = 1 };
    return result;
}

rl_array rl_str_bytes(rl_string s) {
    int64_t *buf = malloc(s.len * sizeof(int64_t));
    for (uint64_t i = 0; i < s.len; i++) {
        buf[i] = (int64_t)(unsigned char)s.data[i];
    }
    return rl_arr_from_vals(buf, s.len, sizeof(int64_t));
}

rl_array rl_str_chars(rl_string s) {
    uint64_t cap = 16;
    char *buf = malloc(cap);
    uint64_t count = 0;
    uint64_t i = 0;
    while (i < s.len) {
        unsigned char c = (unsigned char)s.data[i];
        uint64_t char_len;
        char ch;
        if (c < 0x80) {
            char_len = 1;
            ch = (char)c;
        } else if (c < 0xE0) {
            char_len = 2;
            ch = (char)(c & 0x1F);
        } else if (c < 0xF0) {
            char_len = 3;
            ch = (char)(c & 0x0F);
        } else {
            char_len = 4;
            ch = (char)(c & 0x07);
        }
        for (uint64_t j = 1; j < char_len && i + j < s.len; j++) {
            ch = (ch << 6) | ((unsigned char)s.data[i + j] & 0x3F);
        }
        if (count >= cap) {
            cap *= 2;
            buf = realloc(buf, cap);
        }
        buf[count++] = ch;
        i += char_len;
    }
    rl_array arr;
    arr.data = buf;
    arr.len = count;
    arr.cap = cap;
    arr.elem_size = sizeof(char);
    arr.type_tag = RL_TAG_CHAR;
    return arr;
}

char rl_str_char_at(rl_string s, int64_t index) {
    if (index < 0) return '\0';
    uint64_t pos = 0;
    int64_t char_idx = 0;
    while (pos < s.len) {
        unsigned char c = (unsigned char)s.data[pos];
        uint64_t char_len;
        int64_t codepoint;
        if (c < 0x80) { char_len = 1; codepoint = c; }
        else if (c < 0xE0) { char_len = 2; codepoint = c & 0x1F; }
        else if (c < 0xF0) { char_len = 3; codepoint = c & 0x0F; }
        else { char_len = 4; codepoint = c & 0x07; }
        for (uint64_t j = 1; j < char_len && pos + j < s.len; j++) {
            codepoint = (codepoint << 6) | ((unsigned char)s.data[pos + j] & 0x3F);
        }
        if (char_idx == index) return (char)codepoint;
        pos += char_len;
        char_idx++;
    }
    return '\0';
}

rl_string rl_str_join(rl_array arr, rl_string delim) {
    if (arr.len == 0) {
        rl_string result = { .data = "", .len = 0, .rc = 1 };
        return result;
    }
    uint64_t cap = arr.len * 8 + arr.len * delim.len + 1;
    char *buf = malloc(cap);
    uint64_t w = 0;
    int64_t *elems = (int64_t *)arr.data;
    for (uint64_t i = 0; i < arr.len; i++) {
        if (i > 0) {
            memcpy(buf + w, delim.data, delim.len);
            w += delim.len;
        }
        char tmp[32];
        int len = snprintf(tmp, sizeof(tmp), "%ld", (long)elems[i]);
        memcpy(buf + w, tmp, len);
        w += len;
    }
    buf[w] = '\0';
    rl_string result = { .data = buf, .len = w, .rc = 1 };
    return result;
}

rl_array rl_str_split(rl_string s, rl_string delim) {
    if (delim.len == 0 || s.len == 0) {
        rl_array arr = { .data = NULL, .len = 0, .cap = 0, .elem_size = sizeof(rl_string), .type_tag = RL_TAG_STR };
        return arr;
    }
    uint64_t cap = 16;
    rl_string *buf = malloc(cap * sizeof(rl_string));
    uint64_t count = 0;
    uint64_t pos = 0;
    while (pos <= s.len) {
        uint64_t next = pos;
        bool found = false;
        while (next + delim.len <= s.len) {
            if (memcmp(s.data + next, delim.data, delim.len) == 0) {
                found = true;
                break;
            }
            next++;
        }
        uint64_t seg_len = found ? next - pos : s.len - pos;
        if (count >= cap) {
            cap *= 2;
            buf = realloc(buf, cap * sizeof(rl_string));
        }
        char *seg = malloc(seg_len + 1);
        memcpy(seg, s.data + pos, seg_len);
        seg[seg_len] = '\0';
        buf[count] = (rl_string){ .data = seg, .len = seg_len, .rc = 1 };
        count++;
        if (found) {
            pos = next + delim.len;
        } else {
            break;
        }
    }
    rl_array arr;
    arr.data = buf;
    arr.len = count;
    arr.cap = cap;
    arr.elem_size = sizeof(rl_string);
    arr.type_tag = RL_TAG_STR;
    return arr;
}

// ---- debug ----

void rl_panic(rl_string msg) {
    if (msg.len > 0) {
        fprintf(stderr, "panic: %.*s\n", (int)msg.len, msg.data);
    } else {
        fprintf(stderr, "panic\n");
    }
    exit(1);
}

void rl_unreachable(void) {
    fprintf(stderr, "error: reached unreachable code\n");
    exit(1);
}

void rl_todo(void) {
    fprintf(stderr, "error: not yet implemented\n");
    exit(1);
}

void rl_assert_fail(rl_string label, int64_t a, int64_t b) {
    fprintf(stderr, "assertion failed: %.*s: %ld != %ld\n",
            (int)label.len, label.data, (long)a, (long)b);
    exit(1);
}

void rl_assert_fail_msg(rl_string label, rl_string msg) {
    fprintf(stderr, "assertion failed: %.*s: %.*s\n",
            (int)label.len, label.data, (int)msg.len, msg.data);
    exit(1);
}

rl_string rl_type_of(int64_t type_tag) {
    const char *name;
    switch (type_tag) {
        case 1: name = "int"; break;
        case 2: name = "float"; break;
        case 3: name = "bool"; break;
        case 4: name = "string"; break;
        default: name = "unknown"; break;
    }
    uint64_t len = strlen(name);
    char *buf = malloc(len + 1);
    memcpy(buf, name, len + 1);
    rl_string result = { .data = buf, .len = len, .rc = 1 };
    return result;
}

int64_t rl_dbg_int64(int64_t v) {
    fprintf(stderr, "[dbg] %ld (int)\n", (long)v);
    return v;
}

double rl_dbg_float64(double v) {
    fprintf(stderr, "[dbg] %.15g (float)\n", v);
    return v;
}

bool rl_dbg_bool(bool v) {
    fprintf(stderr, "[dbg] %s (bool)\n", v ? "true" : "false");
    return v;
}

rl_string rl_dbg_str(rl_string v) {
    fprintf(stderr, "[dbg] \"%.*s\" (string)\n", (int)v.len, v.data);
    return v;
}

// ---- path ----

rl_string rl_path_extension(rl_string path) {
    int64_t last_dot = -1;
    for (int64_t i = (int64_t)path.len - 1; i >= 0; i--) {
        if (path.data[i] == '.') { last_dot = i; break; }
    }
    if (last_dot < 0 || (uint64_t)last_dot >= path.len - 1) {
        rl_string result = { .data = "", .len = 0, .rc = 1 };
        return result;
    }
    uint64_t ext_len = path.len - (uint64_t)last_dot - 1;
    char *buf = malloc(ext_len + 1);
    memcpy(buf, path.data + last_dot + 1, ext_len);
    buf[ext_len] = '\0';
    rl_string result = { .data = buf, .len = ext_len, .rc = 1 };
    return result;
}

rl_string rl_path_filename(rl_string path) {
    int64_t last_sep = -1;
    for (int64_t i = (int64_t)path.len - 1; i >= 0; i--) {
        if (path.data[i] == '/') { last_sep = i; break; }
    }
    uint64_t start = (last_sep >= 0) ? (uint64_t)(last_sep + 1) : 0;
    uint64_t len = path.len - start;
    char *buf = malloc(len + 1);
    memcpy(buf, path.data + start, len);
    buf[len] = '\0';
    rl_string result = { .data = buf, .len = len, .rc = 1 };
    return result;
}

rl_string rl_path_parent(rl_string path) {
    int64_t last_sep = -1;
    for (int64_t i = (int64_t)path.len - 1; i >= 0; i--) {
        if (path.data[i] == '/') { last_sep = i; break; }
    }
    if (last_sep < 0) {
        rl_string result = { .data = ".", .len = 1, .rc = 1 };
        return result;
    }
    if (last_sep == 0) {
        rl_string result = { .data = "/", .len = 1, .rc = 1 };
        return result;
    }
    char *buf = malloc((uint64_t)last_sep + 1);
    memcpy(buf, path.data, (uint64_t)last_sep);
    buf[last_sep] = '\0';
    rl_string result = { .data = buf, .len = (uint64_t)last_sep, .rc = 1 };
    return result;
}

rl_string rl_path_stem(rl_string path) {
    int64_t last_sep = -1;
    int64_t last_dot = -1;
    for (int64_t i = (int64_t)path.len - 1; i >= 0; i--) {
        if (path.data[i] == '/' && last_sep < 0) last_sep = i;
        if (path.data[i] == '.' && last_dot < 0) last_dot = i;
    }
    uint64_t start = (last_sep >= 0) ? (uint64_t)(last_sep + 1) : 0;
    uint64_t end = (last_dot > last_sep) ? (uint64_t)last_dot : path.len;
    uint64_t len = end - start;
    char *buf = malloc(len + 1);
    memcpy(buf, path.data + start, len);
    buf[len] = '\0';
    rl_string result = { .data = buf, .len = len, .rc = 1 };
    return result;
}

rl_string rl_path_pop(rl_string path) {
    return rl_path_parent(path);
}

rl_string rl_path_join(rl_string path, rl_string target) {
    if (target.len == 0) {
        char *buf = malloc(path.len + 1);
        memcpy(buf, path.data, path.len);
        buf[path.len] = '\0';
        rl_string result = { .data = buf, .len = path.len, .rc = 1 };
        return result;
    }
    uint64_t total = path.len + 1 + target.len;
    char *buf = malloc(total + 1);
    memcpy(buf, path.data, path.len);
    buf[path.len] = '/';
    memcpy(buf + path.len + 1, target.data, target.len);
    buf[total] = '\0';
    rl_string result = { .data = buf, .len = total, .rc = 1 };
    return result;
}

rl_string rl_path_push(rl_string path, rl_string target) {
    return rl_path_join(path, target);
}

rl_string rl_path_set_extension(rl_string path, rl_string ext) {
    int64_t last_dot = -1;
    for (int64_t i = (int64_t)path.len - 1; i >= 0; i--) {
        if (path.data[i] == '.') { last_dot = i; break; }
    }
    uint64_t base_len = (last_dot >= 0) ? (uint64_t)last_dot : path.len;
    uint64_t total = base_len + 1 + ext.len;
    char *buf = malloc(total + 1);
    memcpy(buf, path.data, base_len);
    buf[base_len] = '.';
    memcpy(buf + base_len + 1, ext.data, ext.len);
    buf[total] = '\0';
    rl_string result = { .data = buf, .len = total, .rc = 1 };
    return result;
}

bool rl_path_is_dir(rl_string path) {
    char buf[path.len + 1];
    memcpy(buf, path.data, path.len);
    buf[path.len] = '\0';
    struct stat st;
    if (stat(buf, &st) != 0) return false;
    return S_ISDIR(st.st_mode);
}

bool rl_path_is_file(rl_string path) {
    char buf[path.len + 1];
    memcpy(buf, path.data, path.len);
    buf[path.len] = '\0';
    struct stat st;
    if (stat(buf, &st) != 0) return false;
    return S_ISREG(st.st_mode);
}

// ---- fs ----

int64_t rl_fs_file_size(rl_string path) {
    char buf[path.len + 1];
    memcpy(buf, path.data, path.len);
    buf[path.len] = '\0';
    struct stat st;
    if (stat(buf, &st) != 0) return -1;
    return (int64_t)st.st_size;
}

int64_t rl_fs_file_modified(rl_string path) {
    char buf[path.len + 1];
    memcpy(buf, path.data, path.len);
    buf[path.len] = '\0';
    struct stat st;
    if (stat(buf, &st) != 0) return -1;
    return (int64_t)st.st_mtime;
}

int64_t rl_fs_copy_file(rl_string src, rl_string dst) {
    char sbuf[src.len + 1];
    memcpy(sbuf, src.data, src.len);
    sbuf[src.len] = '\0';
    char dbuf[dst.len + 1];
    memcpy(dbuf, dst.data, dst.len);
    dbuf[dst.len] = '\0';
    FILE *fin = fopen(sbuf, "rb");
    if (!fin) return -1;
    FILE *fout = fopen(dbuf, "wb");
    if (!fout) { fclose(fin); return -1; }
    char chunk[8192];
    size_t n;
    while ((n = fread(chunk, 1, sizeof(chunk), fin)) > 0) {
        fwrite(chunk, 1, n, fout);
    }
    fclose(fin);
    fclose(fout);
    return 0;
}

int64_t rl_fs_mkdir_all(rl_string path) {
    char buf[path.len + 1];
    memcpy(buf, path.data, path.len);
    buf[path.len] = '\0';
    for (char *p = buf + 1; *p; p++) {
        if (*p == '/') {
            *p = '\0';
            mkdir(buf, 0755);
            *p = '/';
        }
    }
    return mkdir(buf, 0755);
}

int64_t rl_fs_rmdir_all(rl_string path) {
    char buf[path.len + 1];
    memcpy(buf, path.data, path.len);
    buf[path.len] = '\0';
    return rmdir(buf);
}

rl_array rl_fs_list_dir(rl_string path) {
    char buf[path.len + 1];
    memcpy(buf, path.data, path.len);
    buf[path.len] = '\0';
    DIR *d = opendir(buf);
    if (!d) {
        rl_array arr = { .data = NULL, .len = 0, .cap = 0, .elem_size = sizeof(rl_string), .type_tag = RL_TAG_STR };
        return arr;
    }
    uint64_t cap = 16;
    rl_string *entries = malloc(cap * sizeof(rl_string));
    uint64_t count = 0;
    struct dirent *ent;
    while ((ent = readdir(d)) != NULL) {
        if (strcmp(ent->d_name, ".") == 0 || strcmp(ent->d_name, "..") == 0) continue;
        if (count >= cap) {
            cap *= 2;
            entries = realloc(entries, cap * sizeof(rl_string));
        }
        uint64_t name_len = strlen(ent->d_name);
        bool need_sep = path.len > 0 && path.data[path.len - 1] != '/';
        uint64_t full_len = path.len + (need_sep ? 1 : 0) + name_len;
        char *name = malloc(full_len + 1);
        memcpy(name, path.data, path.len);
        if (need_sep) name[path.len] = '/';
        memcpy(name + path.len + (need_sep ? 1 : 0), ent->d_name, name_len + 1);
        entries[count] = (rl_string){ .data = name, .len = full_len, .rc = 1 };
        count++;
    }
    closedir(d);
    rl_array arr;
    arr.data = entries;
    arr.len = count;
    arr.cap = cap;
    arr.elem_size = sizeof(rl_string);
    arr.type_tag = RL_TAG_STR;
    return arr;
}

rl_string rl_fs_rename_file(rl_string path, rl_string new_name) {
    char pbuf[path.len + 1];
    memcpy(pbuf, path.data, path.len);
    pbuf[path.len] = '\0';
    char nbuf[new_name.len + 1];
    memcpy(nbuf, new_name.data, new_name.len);
    nbuf[new_name.len] = '\0';
    rename(pbuf, nbuf);
    return new_name;
}

// ---- process ----

rl_string rl_process_cwd(void) {
    char buf[4096];
    if (getcwd(buf, sizeof(buf)) == NULL) {
        rl_string result = { .data = "", .len = 0, .rc = 1 };
        return result;
    }
    uint64_t len = strlen(buf);
    char *out = malloc(len + 1);
    memcpy(out, buf, len + 1);
    rl_string result = { .data = out, .len = len, .rc = 1 };
    return result;
}

int64_t rl_process_set_cwd(rl_string path) {
    char buf[path.len + 1];
    memcpy(buf, path.data, path.len);
    buf[path.len] = '\0';
    return chdir(buf);
}

rl_string rl_process_exec(rl_string cmd) {
    char buf[cmd.len + 1];
    memcpy(buf, cmd.data, cmd.len);
    buf[cmd.len] = '\0';
    FILE *fp = popen(buf, "r");
    if (!fp) {
        rl_string result = { .data = "", .len = 0, .rc = 1 };
        return result;
    }
    uint64_t cap = 4096;
    char *out = malloc(cap);
    uint64_t len = 0;
    size_t n;
    while ((n = fread(out + len, 1, cap - len, fp)) > 0) {
        len += n;
        if (len >= cap) {
            cap *= 2;
            out = realloc(out, cap);
        }
    }
    pclose(fp);
    out[len] = '\0';
    while (len > 0 && out[len - 1] == '\n') {
        len--;
    }
    rl_string result = { .data = out, .len = len, .rc = 1 };
    return result;
}

int64_t rl_process_exec_code(rl_string cmd) {
    char buf[cmd.len + 1];
    memcpy(buf, cmd.data, cmd.len);
    buf[cmd.len] = '\0';
    return (int64_t)system(buf);
}

rl_array rl_process_exec_lines(rl_string cmd) {
    rl_string output = rl_process_exec(cmd);
    rl_string nl = { .data = "\n", .len = 1, .rc = 1 };
    return rl_str_split(output, nl);
}

rl_string rl_process_with_exec(rl_string env, rl_string cmd) {
    uint64_t total = env.len + 1 + cmd.len;
    char buf[total + 1];
    memcpy(buf, env.data, env.len);
    buf[env.len] = ' ';
    memcpy(buf + env.len + 1, cmd.data, cmd.len);
    buf[total] = '\0';
    rl_string combined = { .data = buf, .len = total, .rc = 0 };
    return rl_process_exec(combined);
}

int64_t rl_process_with_exec_code(rl_string env, rl_string cmd) {
    uint64_t total = env.len + 1 + cmd.len;
    char buf[total + 1];
    memcpy(buf, env.data, env.len);
    buf[env.len] = ' ';
    memcpy(buf + env.len + 1, cmd.data, cmd.len);
    buf[total] = '\0';
    rl_string combined = { .data = buf, .len = total, .rc = 0 };
    return rl_process_exec_code(combined);
}

rl_array rl_process_with_exec_lines(rl_string env, rl_string cmd) {
    uint64_t total = env.len + 1 + cmd.len;
    char buf[total + 1];
    memcpy(buf, env.data, env.len);
    buf[env.len] = ' ';
    memcpy(buf + env.len + 1, cmd.data, cmd.len);
    buf[total] = '\0';
    rl_string combined = { .data = buf, .len = total, .rc = 0 };
    return rl_process_exec_lines(combined);
}

static int _rl_stored_argc = 0;
static char **_rl_stored_argv = NULL;

void rl_store_args(int argc, char **argv) {
    _rl_stored_argc = argc;
    _rl_stored_argv = argv;
}

rl_array rl_process_args(void) {
    if (!_rl_stored_argv) {
        rl_array arr = { .data = NULL, .len = 0, .cap = 0, .elem_size = sizeof(rl_string), .type_tag = RL_TAG_STR };
        return arr;
    }
    uint64_t len = (uint64_t)(_rl_stored_argc > 0 ? _rl_stored_argc - 1 : 0);
    rl_string *buf = malloc(len * sizeof(rl_string));
    for (uint64_t i = 0; i < len; i++) {
        const char *s = _rl_stored_argv[i + 1];
        uint64_t slen = strlen(s);
        char *sdup = malloc(slen + 1);
        memcpy(sdup, s, slen + 1);
        buf[i] = (rl_string){ .data = sdup, .len = slen };
    }
    rl_array arr = { .data = buf, .len = len, .cap = len, .elem_size = sizeof(rl_string), .type_tag = RL_TAG_STR };
    return arr;
}

// ---- time ----

rl_string rl_time_format_time(int64_t timestamp, rl_string pattern) {
    time_t t = (time_t)timestamp;
    struct tm *tm = gmtime(&t);
    char buf[256];
    strftime(buf, sizeof(buf), pattern.data, tm);
    uint64_t len = strlen(buf);
    char *out = malloc(len + 1);
    memcpy(out, buf, len + 1);
    rl_string result = { .data = out, .len = len, .rc = 1 };
    return result;
}

rl_string rl_time_format_date_str(int64_t timestamp) {
    time_t t = (time_t)timestamp;
    struct tm *tm = gmtime(&t);
    char buf[64];
    strftime(buf, sizeof(buf), "%Y-%m-%d", tm);
    uint64_t len = strlen(buf);
    char *out = malloc(len + 1);
    memcpy(out, buf, len + 1);
    rl_string result = { .data = out, .len = len, .rc = 1 };
    return result;
}

rl_string rl_time_format_time_str(int64_t timestamp) {
    time_t t = (time_t)timestamp;
    struct tm *tm = gmtime(&t);
    char buf[64];
    strftime(buf, sizeof(buf), "%H:%M:%S", tm);
    uint64_t len = strlen(buf);
    char *out = malloc(len + 1);
    memcpy(out, buf, len + 1);
    rl_string result = { .data = out, .len = len, .rc = 1 };
    return result;
}

rl_array rl_time_parts(int64_t timestamp) {
    time_t t = (time_t)timestamp;
    struct tm *tm = gmtime(&t);
    int64_t parts[6] = {
        tm->tm_year + 1900,
        tm->tm_mon + 1,
        tm->tm_mday,
        tm->tm_hour,
        tm->tm_min,
        tm->tm_sec
    };
    return rl_arr_from_vals(parts, 6, sizeof(int64_t));
}

// ---- io ----

rl_result rl_io_read_file(rl_string path) {
    char buf[path.len + 1];
    memcpy(buf, path.data, path.len);
    buf[path.len] = '\0';
    FILE *f = fopen(buf, "rb");
    if (!f) {
        return rl_err_msg(rl_str_literal("failed to open file for reading", 31));
    }
    fseek(f, 0, SEEK_END);
    long size = ftell(f);
    fseek(f, 0, SEEK_SET);
    if (size <= 0) {
        fclose(f);
        return rl_ok_str(rl_str_literal("", 0));
    }
    char *out = malloc((uint64_t)size);
    size_t n = fread(out, 1, (uint64_t)size, f);
    fclose(f);
    rl_string result = { .data = out, .len = (uint64_t)n, .rc = 1 };
    return rl_ok_str(result);
}

rl_result rl_io_read_lines(rl_string path) {
    rl_result content_r = rl_io_read_file(path);
    if (!content_r.is_ok) return content_r;
    rl_string nl = { .data = "\n", .len = 1, .rc = 1 };
    rl_array lines = rl_str_split(content_r.data.str, nl);
    return rl_ok_arr(lines);
}

rl_string rl_io_read(void) {
    uint64_t cap = 256;
    char *buf = malloc(cap);
    uint64_t len = 0;
    int c;
    while ((c = fgetc(stdin)) != EOF && c != '\n') {
        if (len >= cap) {
            cap *= 2;
            buf = realloc(buf, cap);
        }
        buf[len++] = (char)c;
    }
    buf[len] = '\0';
    rl_string result = { .data = buf, .len = len, .rc = 1 };
    return result;
}

int64_t rl_io_read_int(void) {
    int64_t v = 0;
    scanf("%ld", &v);
    return v;
}

double rl_io_read_float(void) {
    double v = 0.0;
    scanf("%lf", &v);
    return v;
}

rl_result rl_io_write_file(rl_string path, rl_string content) {
    char buf[path.len + 1];
    memcpy(buf, path.data, path.len);
    buf[path.len] = '\0';
    FILE *f = fopen(buf, "wb");
    if (!f) return rl_err_msg(rl_str_literal("failed to open file for writing", 30));
    fwrite(content.data, 1, content.len, f);
    fclose(f);
    return rl_ok_null();
}

rl_result rl_io_append_file(rl_string path, rl_string content) {
    char buf[path.len + 1];
    memcpy(buf, path.data, path.len);
    buf[path.len] = '\0';
    FILE *f = fopen(buf, "ab");
    if (!f) return rl_err_msg(rl_str_literal("failed to open file for appending", 32));
    fwrite(content.data, 1, content.len, f);
    fclose(f);
    return rl_ok_null();
}

int64_t rl_io_delete_file(rl_string path) {
    char buf[path.len + 1];
    memcpy(buf, path.data, path.len);
    buf[path.len] = '\0';
    return remove(buf);
}

void rl_io_eprint(rl_string msg) {
    fprintf(stderr, "%.*s", (int)msg.len, msg.data);
}

void rl_io_eprintln(rl_string msg) {
    fprintf(stderr, "%.*s\n", (int)msg.len, msg.data);
}

// ---- types ----

rl_string rl_types_to_string(int64_t v) {
    char buf[32];
    int len = snprintf(buf, sizeof(buf), "%ld", (long)v);
    char *out = malloc(len + 1);
    memcpy(out, buf, len + 1);
    rl_string result = { .data = out, .len = (uint64_t)len, .rc = 1 };
    return result;
}

rl_string rl_types_to_bin(int64_t v) {
    if (v == 0) {
        rl_string result = { .data = "0", .len = 1, .rc = 1 };
        return result;
    }
    char buf[65];
    int i = 64;
    buf[i] = '\0';
    uint64_t uv = (uint64_t)v;
    while (uv > 0) {
        buf[--i] = (uv & 1) ? '1' : '0';
        uv >>= 1;
    }
    uint64_t len = 64 - (uint64_t)i;
    char *out = malloc(len + 1);
    memcpy(out, buf + i, len + 1);
    rl_string result = { .data = out, .len = len, .rc = 1 };
    return result;
}

rl_string rl_types_to_hex(int64_t v) {
    char buf[32];
    int len = snprintf(buf, sizeof(buf), "%lx", (unsigned long)v);
    char *out = malloc(len + 1);
    memcpy(out, buf, len + 1);
    rl_string result = { .data = out, .len = (uint64_t)len, .rc = 1 };
    return result;
}

rl_string rl_types_to_oct(int64_t v) {
    char buf[32];
    int len = snprintf(buf, sizeof(buf), "%lo", (unsigned long)v);
    char *out = malloc(len + 1);
    memcpy(out, buf, len + 1);
    rl_string result = { .data = out, .len = (uint64_t)len, .rc = 1 };
    return result;
}

// ---- random ----

static int rl_rand_initialized = 0;

static void rl_rand_ensure_init(void) {
    if (!rl_rand_initialized) {
        srand((unsigned int)time(NULL));
        rl_rand_initialized = 1;
    }
}

int64_t rl_rand_int(void) {
    rl_rand_ensure_init();
    return (int64_t)rand();
}

double rl_rand_float(void) {
    rl_rand_ensure_init();
    return (double)rand() / (double)RAND_MAX;
}

bool rl_rand_bool(void) {
    rl_rand_ensure_init();
    return rand() % 2 == 0;
}

bool rl_rand_bool_weighted(double weight) {
    rl_rand_ensure_init();
    return rl_rand_float() < weight;
}

char rl_rand_char(void) {
    rl_rand_ensure_init();
    return (char)('a' + rand() % 26);
}

int64_t rl_rand_byte(void) {
    rl_rand_ensure_init();
    return (int64_t)(rand() % 256);
}

int64_t rl_rand_int_range(int64_t min, int64_t max) {
    rl_rand_ensure_init();
    if (min >= max) return min;
    return min + (int64_t)(rand() % (uint64_t)(max - min));
}

double rl_rand_float_range(double min, double max) {
    rl_rand_ensure_init();
    return min + (max - min) * rl_rand_float();
}

int64_t rl_rand_dice(int64_t sides) {
    rl_rand_ensure_init();
    if (sides <= 0) return 0;
    return 1 + (int64_t)(rand() % (uint64_t)sides);
}

int64_t rl_rand_range(int64_t stop) {
    rl_rand_ensure_init();
    if (stop <= 0) return 0;
    return (int64_t)(rand() % (uint64_t)stop);
}

int64_t rl_rand_range_step(int64_t start, int64_t stop, int64_t step) {
    rl_rand_ensure_init();
    if (step == 0 || (start < stop && step < 0) || (start > stop && step > 0)) return start;
    uint64_t range;
    if (step > 0) {
        range = (uint64_t)((stop - start + step - 1) / step);
    } else {
        range = (uint64_t)((start - stop - step - 1) / (-step));
    }
    if (range == 0) return start;
    return start + (int64_t)((uint64_t)(rand() % (int)range) * (uint64_t)step);
}

rl_string rl_rand_string(int64_t count) {
    rl_rand_ensure_init();
    if (count <= 0) {
        rl_string result = { .data = "", .len = 0, .rc = 1 };
        return result;
    }
    char *buf = malloc((uint64_t)count + 1);
    for (int64_t i = 0; i < count; i++) {
        buf[i] = (char)('a' + rand() % 26);
    }
    buf[count] = '\0';
    rl_string result = { .data = buf, .len = (uint64_t)count, .rc = 1 };
    return result;
}

// ---- collections (rl_string key wrappers) ----

rl_result rl_set_add_s(rl_set *s, rl_value value) {
    rl_set_add(s, value);
    return rl_ok_i64(1);
}

rl_result rl_set_remove_s(rl_set *s, rl_value value) {
    rl_set_remove(s, value);
    return rl_ok_i64(1);
}

rl_result rl_set_contains_s(rl_set s, rl_value value) {
    return rl_ok_bool(rl_set_contains(s, value));
}

rl_array rl_set_to_array(rl_set s) {
    rl_value *buf = malloc(s.len * sizeof(rl_value));
    memcpy(buf, s.data, s.len * sizeof(rl_value));
    rl_array arr = { .data = buf, .len = s.len, .cap = s.len, .elem_size = sizeof(rl_value), .type_tag = RL_TAG_I64 };
    return arr;
}

rl_result rl_map_contains_s(rl_map m, rl_string key) {
    char buf[key.len + 1];
    memcpy(buf, key.data, key.len);
    buf[key.len] = '\0';
    return rl_ok_bool(rl_map_contains(m, buf));
}

rl_result rl_map_remove_s(rl_map m, rl_string key) {
    char buf[key.len + 1];
    memcpy(buf, key.data, key.len);
    buf[key.len] = '\0';
    rl_map_remove(&m, buf);
    return rl_ok(m);
}

rl_result rl_map_get_s(rl_map m, rl_string key) {
    char buf[key.len + 1];
    memcpy(buf, key.data, key.len);
    buf[key.len] = '\0';
    if (!rl_map_contains(m, buf)) {
        return rl_err_msg(rl_str_literal("key not found in map", 20));
    }
    rl_value v = rl_map_get(m, buf);
    switch (v.tag) {
        case RL_VTAG_NULL: return rl_ok_null();
        case RL_VTAG_I64: return rl_ok_i64(v.data.i64);
        case RL_VTAG_F64: return rl_ok_f64(v.data.f64);
        case RL_VTAG_BOOL: return rl_ok_bool(v.data.boolean);
        case RL_VTAG_STR: return rl_ok_str(v.data.str);
        case RL_VTAG_ARR: return rl_ok_arr(v.data.arr);
        case RL_VTAG_CLOSURE: {
            rl_result r = { .is_ok = true, .tag = RL_TAG_CLOSURE, .data.closure = v.data.closure, .err_code = 0 };
            return r;
        }
        default: return rl_ok_i64(v.data.i64);
    }
}

rl_array rl_map_keys_s(rl_map m) {
    int64_t *buf = malloc(m.len * sizeof(int64_t));
    for (uint64_t i = 0; i < m.len; i++) {
        uint64_t len = strlen(m.entries[i].key);
        char *s = malloc(len + 1);
        memcpy(s, m.entries[i].key, len + 1);
        buf[i] = (int64_t)(uintptr_t)s;
    }
    return rl_arr_from_vals(buf, m.len, sizeof(int64_t));
}

rl_array rl_map_values_s(rl_map m) {
    rl_value *buf = malloc(m.len * sizeof(rl_value));
    for (uint64_t i = 0; i < m.len; i++) {
        buf[i] = m.entries[i].value;
    }
    rl_array arr = { .data = buf, .len = m.len, .cap = m.len, .elem_size = sizeof(rl_value), .type_tag = RL_TAG_I64 };
    return arr;
}

rl_map rl_map_merge_s(rl_map a, rl_map b) {
    rl_map result;
    result.len = a.len;
    result.cap = a.cap ? a.cap : 8;
    if (result.cap < a.len + b.len) result.cap = (a.len + b.len) * 2;
    result.entries = malloc(result.cap * sizeof(rl_map_entry));
    memcpy(result.entries, a.entries, a.len * sizeof(rl_map_entry));
    for (uint64_t i = 0; i < b.len; i++) {
        bool found = false;
        for (uint64_t j = 0; j < result.len; j++) {
            if (strcmp(result.entries[j].key, b.entries[i].key) == 0) {
                result.entries[j].value = b.entries[i].value;
                found = true;
                break;
            }
        }
        if (!found) {
            if (result.len >= result.cap) {
                result.cap *= 2;
                result.entries = realloc(result.entries, result.cap * sizeof(rl_map_entry));
            }
            result.entries[result.len++] = b.entries[i];
        }
    }
    return result;
}

rl_array rl_map_to_array_s(rl_map m) {
    rl_value *buf = malloc(m.len * sizeof(rl_value));
    for (uint64_t i = 0; i < m.len; i++) {
        buf[i] = m.entries[i].value;
    }
    rl_array arr = { .data = buf, .len = m.len, .cap = m.len, .elem_size = sizeof(rl_value), .type_tag = RL_TAG_I64 };
    return arr;
}

// ---- array (generic) ----

rl_result rl_arr_push(rl_array a, int64_t v) {
    uint64_t new_len = a.len + 1;
    int64_t *buf = malloc(new_len * sizeof(int64_t));
    if (a.data) memcpy(buf, a.data, a.len * sizeof(int64_t));
    buf[a.len] = v;
    return rl_ok_arr(rl_arr_from_vals(buf, new_len, sizeof(int64_t)));
}

rl_result rl_arr_pop(rl_array a) {
    if (a.len == 0) return rl_make_err(-1, "pop from empty array");
    return rl_ok_i64(((int64_t *)a.data)[a.len - 1]);
}

rl_result rl_arr_insert(rl_array a, int64_t idx, int64_t v) {
    if (idx < 0) idx = 0;
    if ((uint64_t)idx > a.len) idx = (int64_t)a.len;
    uint64_t new_len = a.len + 1;
    int64_t *buf = malloc(new_len * sizeof(int64_t));
    int64_t *src = (int64_t *)a.data;
    if (idx > 0 && src) memcpy(buf, src, (uint64_t)idx * sizeof(int64_t));
    buf[idx] = v;
    if (src && (uint64_t)idx < a.len) memcpy(buf + idx + 1, src + idx, (a.len - (uint64_t)idx) * sizeof(int64_t));
    return rl_ok_arr(rl_arr_from_vals(buf, new_len, sizeof(int64_t)));
}

rl_result rl_arr_remove(rl_array a, int64_t idx) {
    if (idx < 0 || (uint64_t)idx >= a.len) return rl_make_err(-1, "index out of bounds");
    int64_t *src = (int64_t *)a.data;
    uint64_t new_len = a.len - 1;
    int64_t *buf = malloc(new_len * sizeof(int64_t));
    if (idx > 0) memcpy(buf, src, (uint64_t)idx * sizeof(int64_t));
    if ((uint64_t)idx < a.len - 1) memcpy(buf + idx, src + idx + 1, (a.len - (uint64_t)idx - 1) * sizeof(int64_t));
    return rl_ok_arr(rl_arr_from_vals(buf, new_len, sizeof(int64_t)));
}

rl_array rl_arr_reverse(rl_array a) {
    int64_t *src = (int64_t *)a.data;
    int64_t *buf = malloc(a.len * sizeof(int64_t));
    for (uint64_t i = 0; i < a.len; i++) {
        buf[i] = src[a.len - 1 - i];
    }
    return rl_arr_from_vals(buf, a.len, sizeof(int64_t));
}

rl_array rl_arr_concat(rl_array a, rl_array b) {
    uint64_t new_len = a.len + b.len;
    int64_t *buf = malloc(new_len * sizeof(int64_t));
    if (a.data) memcpy(buf, a.data, a.len * sizeof(int64_t));
    if (b.data) memcpy(buf + a.len, b.data, b.len * sizeof(int64_t));
    return rl_arr_from_vals(buf, new_len, sizeof(int64_t));
}

rl_result rl_arr_first(rl_array a) {
    if (a.len == 0) return rl_make_err(-1, "first of empty array");
    return rl_ok_i64(((int64_t *)a.data)[0]);
}

rl_result rl_arr_last(rl_array a) {
    if (a.len == 0) return rl_make_err(-1, "last of empty array");
    return rl_ok_i64(((int64_t *)a.data)[a.len - 1]);
}

rl_array rl_arr_unique(rl_array a) {
    int64_t *src = (int64_t *)a.data;
    int64_t *buf = malloc(a.len * sizeof(int64_t));
    uint64_t w = 0;
    for (uint64_t i = 0; i < a.len; i++) {
        bool found = false;
        for (uint64_t j = 0; j < w; j++) {
            if (buf[j] == src[i]) { found = true; break; }
        }
        if (!found) buf[w++] = src[i];
    }
    return rl_arr_from_vals(buf, w, sizeof(int64_t));
}

rl_array rl_arr_slice(rl_array a, int64_t start, int64_t end) {
    if (start < 0) start = 0;
    if (end > (int64_t)a.len) end = (int64_t)a.len;
    if (start >= end) {
        return rl_arr_from_vals(NULL, 0, sizeof(int64_t));
    }
    uint64_t len = (uint64_t)(end - start);
    int64_t *buf = malloc(len * sizeof(int64_t));
    memcpy(buf, (int64_t *)a.data + start, len * sizeof(int64_t));
    return rl_arr_from_vals(buf, len, sizeof(int64_t));
}

rl_result rl_arr_contains(rl_array a, int64_t v) {
    int64_t *src = (int64_t *)a.data;
    for (uint64_t i = 0; i < a.len; i++) {
        if (src[i] == v) return rl_ok_bool(true);
    }
    return rl_ok_bool(false);
}

rl_result rl_arr_index_of(rl_array a, int64_t v) {
    int64_t *src = (int64_t *)a.data;
    for (uint64_t i = 0; i < a.len; i++) {
        if (src[i] == v) return rl_ok_i64((int64_t)i);
    }
    return rl_ok_i64(-1);
}

rl_array rl_arr_fill(int64_t v, int64_t count) {
    if (count <= 0) return rl_arr_from_vals(NULL, 0, sizeof(int64_t));
    int64_t *buf = malloc((uint64_t)count * sizeof(int64_t));
    for (int64_t i = 0; i < count; i++) buf[i] = v;
    return rl_arr_from_vals(buf, (uint64_t)count, sizeof(int64_t));
}

rl_result rl_arr_range(int64_t start, int64_t end, int64_t step) {
    if (step == 0) {
        return rl_make_err(-1, "arr_range: step must be positive, got 0");
    }
    if (step < 0) {
        char msg[64];
        snprintf(msg, sizeof(msg), "arr_range: step must be positive, got %ld", (long)step);
        uint64_t len = strlen(msg);
        char *buf = malloc(len + 1);
        memcpy(buf, msg, len + 1);
        rl_string s = { .data = buf, .len = len, .rc = 1 };
        rl_result r = { .is_ok = false, .tag = RL_TAG_STR, .data.str = s, .err_code = -1 };
        return r;
    }
    uint64_t cap = 16;
    int64_t *buf = malloc(cap * sizeof(int64_t));
    uint64_t count = 0;
    for (int64_t i = start; i < end; i += step) {
        if (count >= cap) { cap *= 2; buf = realloc(buf, cap * sizeof(int64_t)); }
        buf[count++] = i;
    }
    return rl_ok_arr(rl_arr_from_vals(buf, count, sizeof(int64_t)));
}

rl_result rl_arr_sum(rl_array a) {
    int64_t *src = (int64_t *)a.data;
    int64_t sum = 0;
    for (uint64_t i = 0; i < a.len; i++) sum += src[i];
    return rl_ok_i64(sum);
}

rl_result rl_arr_product(rl_array a) {
    int64_t *src = (int64_t *)a.data;
    int64_t prod = 1;
    for (uint64_t i = 0; i < a.len; i++) prod *= src[i];
    return rl_ok_i64(prod);
}

rl_result rl_arr_max(rl_array a) {
    if (a.len == 0) return rl_make_err(-1, "max of empty array");
    int64_t *src = (int64_t *)a.data;
    int64_t max = src[0];
    for (uint64_t i = 1; i < a.len; i++) {
        if (src[i] > max) max = src[i];
    }
    return rl_ok_i64(max);
}

rl_result rl_arr_min(rl_array a) {
    if (a.len == 0) return rl_make_err(-1, "min of empty array");
    int64_t *src = (int64_t *)a.data;
    int64_t min = src[0];
    for (uint64_t i = 1; i < a.len; i++) {
        if (src[i] < min) min = src[i];
    }
    return rl_ok_i64(min);
}

static int rl_arr_cmp_i64(const void *a, const void *b) {
    int64_t va = *(const int64_t *)a;
    int64_t vb = *(const int64_t *)b;
    return (va > vb) - (va < vb);
}

static int rl_arr_cmp_f64(const void *a, const void *b) {
    double va = *(const double *)a;
    double vb = *(const double *)b;
    return (va > vb) - (va < vb);
}

static int rl_arr_cmp_str(const void *a, const void *b) {
    rl_string sa = *(const rl_string *)a;
    rl_string sb = *(const rl_string *)b;
    uint64_t min_len = sa.len < sb.len ? sa.len : sb.len;
    int cmp = memcmp(sa.data, sb.data, min_len);
    if (cmp != 0) return cmp;
    return (sa.len > sb.len) - (sa.len < sb.len);
}

rl_array rl_arr_sort(rl_array a) {
    if (a.len == 0) return a;
    switch (a.type_tag) {
        case RL_TAG_F64: {
            double *buf = malloc(a.len * sizeof(double));
            if (a.data) memcpy(buf, a.data, a.len * sizeof(double));
            qsort(buf, a.len, sizeof(double), rl_arr_cmp_f64);
            rl_array result = { .data = buf, .len = a.len, .cap = a.len, .elem_size = sizeof(double), .type_tag = RL_TAG_F64 };
            return result;
        }
        case RL_TAG_STR: {
            rl_string *buf = malloc(a.len * sizeof(rl_string));
            if (a.data) memcpy(buf, a.data, a.len * sizeof(rl_string));
            qsort(buf, a.len, sizeof(rl_string), rl_arr_cmp_str);
            rl_array result = { .data = buf, .len = a.len, .cap = a.len, .elem_size = sizeof(rl_string), .type_tag = RL_TAG_STR };
            return result;
        }
        default: {
            int64_t *buf = malloc(a.len * sizeof(int64_t));
            if (a.data) memcpy(buf, a.data, a.len * sizeof(int64_t));
            qsort(buf, a.len, sizeof(int64_t), rl_arr_cmp_i64);
            return rl_arr_from_vals(buf, a.len, sizeof(int64_t));
        }
    }
}

rl_array rl_arr_flatten(rl_array a) {
    int64_t *buf = malloc(a.len * sizeof(int64_t));
    if (a.data) memcpy(buf, a.data, a.len * sizeof(int64_t));
    return rl_arr_from_vals(buf, a.len, sizeof(int64_t));
}

// ---- closure-consuming array functions ----

rl_result rl_arr_filter_closure(rl_array arr, rl_closure pred) {
    uint64_t cap = 16;
    int64_t *buf = malloc(cap * sizeof(int64_t));
    uint64_t count = 0;
    int64_t *elems = (int64_t *)arr.data;
    for (uint64_t i = 0; i < arr.len; i++) {
        rl_result arg = rl_ok_i64(elems[i]);
        rl_result keep = rl_closure_call(pred, &arg, 1);
        bool truth = false;
        if (keep.is_ok) {
            if (keep.tag == RL_TAG_BOOL) truth = keep.data.boolean;
            else if (keep.tag == RL_TAG_I64) truth = (keep.data.i64 != 0);
            else if (keep.tag == RL_TAG_NULL) truth = false;
        }
        if (truth) {
            if (count >= cap) { cap *= 2; buf = realloc(buf, cap * sizeof(int64_t)); }
            buf[count++] = elems[i];
        }
    }
    return rl_ok_arr(rl_arr_from_vals(buf, count, sizeof(int64_t)));
}

rl_result rl_arr_map_closure(rl_array arr, rl_closure fn) {
    uint64_t cap = arr.len > 0 ? arr.len : 16;
    int64_t *buf = malloc(cap * sizeof(int64_t));
    uint64_t count = 0;
    int64_t *elems = (int64_t *)arr.data;
    for (uint64_t i = 0; i < arr.len; i++) {
        rl_result arg = rl_ok_i64(elems[i]);
        rl_result mapped = rl_closure_call(fn, &arg, 1);
        if (count >= cap) { cap *= 2; buf = realloc(buf, cap * sizeof(int64_t)); }
        if (mapped.is_ok) {
            switch (mapped.tag) {
                case RL_TAG_I64: buf[count++] = mapped.data.i64; break;
                case RL_TAG_F64: { double d = mapped.data.f64; int64_t v; memcpy(&v, &d, sizeof(v)); buf[count++] = v; break; }
                case RL_TAG_BOOL: buf[count++] = mapped.data.boolean ? 1 : 0; break;
                case RL_TAG_CHAR: buf[count++] = mapped.data.i64; break;
                default: buf[count++] = 0; break;
            }
        } else {
            buf[count++] = 0;
        }
    }
    return rl_ok_arr(rl_arr_from_vals(buf, count, sizeof(int64_t)));
}

rl_result rl_arr_find_closure(rl_array arr, rl_closure pred) {
    int64_t *elems = (int64_t *)arr.data;
    for (uint64_t i = 0; i < arr.len; i++) {
        rl_result arg = rl_ok_i64(elems[i]);
        rl_result found = rl_closure_call(pred, &arg, 1);
        if (found.is_ok && found.tag == RL_TAG_BOOL && found.data.boolean) {
            return rl_ok_i64(elems[i]);
        }
    }
    return rl_err_msg(rl_str_literal("not found", 9));
}

rl_result rl_arr_reduce_closure(rl_array arr, rl_closure fn, rl_result init) {
    int64_t *elems = (int64_t *)arr.data;
    rl_result acc = init;
    for (uint64_t i = 0; i < arr.len; i++) {
        rl_result args[2] = { acc, rl_ok_i64(elems[i]) };
        acc = rl_closure_call(fn, args, 2);
    }
    return acc;
}

rl_result rl_arr_find_index_closure(rl_array arr, rl_closure pred) {
    int64_t *elems = (int64_t *)arr.data;
    for (uint64_t i = 0; i < arr.len; i++) {
        rl_result arg = rl_ok_i64(elems[i]);
        rl_result found = rl_closure_call(pred, &arg, 1);
        if (found.is_ok && ((found.tag == RL_TAG_BOOL && found.data.boolean) || (found.tag == RL_TAG_I64 && found.data.i64 != 0))) {
            return rl_ok_i64((int64_t)i);
        }
    }
    return rl_ok_i64((int64_t)-1);
}

rl_result rl_arr_all_closure(rl_array arr, rl_closure pred) {
    int64_t *elems = (int64_t *)arr.data;
    for (uint64_t i = 0; i < arr.len; i++) {
        rl_result arg = rl_ok_i64(elems[i]);
        rl_result ok = rl_closure_call(pred, &arg, 1);
        bool truth = false;
        if (ok.is_ok) {
            if (ok.tag == RL_TAG_BOOL) truth = ok.data.boolean;
            else if (ok.tag == RL_TAG_I64) truth = (ok.data.i64 != 0);
        }
        if (!truth) return rl_ok_bool(false);
    }
    return rl_ok_bool(true);
}

rl_result rl_arr_any_closure(rl_array arr, rl_closure pred) {
    int64_t *elems = (int64_t *)arr.data;
    for (uint64_t i = 0; i < arr.len; i++) {
        rl_result arg = rl_ok_i64(elems[i]);
        rl_result ok = rl_closure_call(pred, &arg, 1);
        bool truth = false;
        if (ok.is_ok) {
            if (ok.tag == RL_TAG_BOOL) truth = ok.data.boolean;
            else if (ok.tag == RL_TAG_I64) truth = (ok.data.i64 != 0);
        }
        if (truth) return rl_ok_bool(true);
    }
    return rl_ok_bool(false);
}

rl_result rl_arr_for_each_closure(rl_array arr, rl_closure fn) {
    int64_t *elems = (int64_t *)arr.data;
    for (uint64_t i = 0; i < arr.len; i++) {
        rl_result arg = rl_ok_i64(elems[i]);
        rl_closure_call(fn, &arg, 1);
    }
    return rl_ok_null();
}

rl_result rl_arr_flat_map_closure(rl_array arr, rl_closure fn) {
    uint64_t cap = 16;
    int64_t *buf = malloc(cap * sizeof(int64_t));
    uint64_t count = 0;
    int64_t *elems = (int64_t *)arr.data;
    for (uint64_t i = 0; i < arr.len; i++) {
        rl_result arg = rl_ok_i64(elems[i]);
        rl_result mapped = rl_closure_call(fn, &arg, 1);
        if (mapped.is_ok && mapped.tag == RL_TAG_ARR) {
            rl_array inner = mapped.data.arr;
            int64_t *inner_elems = (int64_t *)inner.data;
            for (uint64_t j = 0; j < inner.len; j++) {
                if (count >= cap) { cap *= 2; buf = realloc(buf, cap * sizeof(int64_t)); }
                buf[count++] = inner_elems[j];
            }
        }
    }
    return rl_ok_arr(rl_arr_from_vals(buf, count, sizeof(int64_t)));
}

rl_result rl_arr_sort_by_closure(rl_array arr, rl_closure cmp) {
    // Copy the array data for sorting
    int64_t *elems = (int64_t *)arr.data;
    uint64_t len = arr.len;
    int64_t *buf = malloc(len * sizeof(int64_t));
    if (len > 0 && elems) memcpy(buf, elems, len * sizeof(int64_t));

    // Simple insertion sort using the comparator closure
    for (uint64_t i = 1; i < len; i++) {
        int64_t key = buf[i];
        int64_t j = (int64_t)i - 1;
        while (j >= 0) {
            rl_result args[2] = { rl_ok_i64(buf[j]), rl_ok_i64(key) };
            rl_result cmp_result = rl_closure_call(cmp, args, 2);
            // If cmp(a, b) > 0, swap (ascending order)
            if (cmp_result.is_ok && cmp_result.tag == RL_TAG_I64 && cmp_result.data.i64 > 0) {
                buf[j + 1] = buf[j];
                j--;
            } else {
                break;
            }
        }
        buf[j + 1] = key;
    }
    return rl_ok_arr(rl_arr_from_vals(buf, len, sizeof(int64_t)));
}

rl_result rl_result_map_closure(rl_result val, rl_closure fn) {
    if (!val.is_ok) return val;
    switch (val.tag) {
        case RL_TAG_I64: {
            rl_result arg = rl_ok_i64(val.data.i64);
            return rl_closure_call(fn, &arg, 1);
        }
        case RL_TAG_F64: {
            rl_result arg = rl_ok_f64(val.data.f64);
            return rl_closure_call(fn, &arg, 1);
        }
        case RL_TAG_BOOL: {
            rl_result arg = rl_ok_bool(val.data.boolean);
            return rl_closure_call(fn, &arg, 1);
        }
        case RL_TAG_STR: {
            rl_result arg = rl_ok_str(val.data.str);
            return rl_closure_call(fn, &arg, 1);
        }
        case RL_TAG_ARR: {
            rl_result arg = rl_ok_arr(val.data.arr);
            return rl_closure_call(fn, &arg, 1);
        }
        default: {
            rl_result arg = rl_ok_null();
            return rl_closure_call(fn, &arg, 1);
        }
    }
}

rl_result rl_result_map_err_closure(rl_result val, rl_closure fn) {
    if (val.is_ok) return val;
    switch (val.tag) {
        case RL_TAG_I64: {
            rl_result arg = rl_ok_i64(val.data.i64);
            return rl_err(rl_unwrap_i64(rl_closure_call(fn, &arg, 1)));
        }
        case RL_TAG_STR: {
            rl_result arg = rl_ok_str(val.data.str);
            return rl_closure_call(fn, &arg, 1);
        }
        default: {
            rl_result arg = rl_ok_i64(val.err_code);
            return rl_err(rl_unwrap_i64(rl_closure_call(fn, &arg, 1)));
        }
    }
}

rl_result rl_bench_closure(rl_closure fn, int64_t iterations) {
    struct timespec start, end;
    clock_gettime(CLOCK_MONOTONIC, &start);
    for (int64_t i = 0; i < iterations; i++) {
        rl_closure_call(fn, NULL, 0);
    }
    clock_gettime(CLOCK_MONOTONIC, &end);
    double elapsed = (end.tv_sec - start.tv_sec) + (end.tv_nsec - start.tv_nsec) / 1e9;
    return rl_ok_f64(elapsed / (double)iterations);
}

// ---- terminal (ANSI escape codes) ----

#include <sys/ioctl.h>
#include <sys/select.h>
#include <unistd.h>
#include <stdio.h>

rl_result rl_term_enter(void) {
    printf("\x1b[?1049h");
    return rl_ok_null();
}

rl_result rl_term_leave(void) {
    printf("\x1b[?1049l");
    return rl_ok_null();
}

rl_result rl_term_clear(void) {
    printf("\x1b[2J\x1b[H");
    return rl_ok_null();
}

rl_result rl_term_clear_line(void) {
    printf("\x1b[2K");
    return rl_ok_null();
}

rl_result rl_term_move(int64_t col, int64_t row) {
    printf("\x1b[%ld;%ldH", (long)row + 1, (long)col + 1);
    return rl_ok_null();
}

rl_result rl_term_move_to_col(int64_t col) {
    printf("\x1b[%ldG", (long)col + 1);
    return rl_ok_null();
}

rl_result rl_term_move_to_row(int64_t row) {
    printf("\x1b[%ld;d", (long)row + 1);
    return rl_ok_null();
}

rl_result rl_term_move_up(int64_t n) {
    printf("\x1b[%ldA", (long)n);
    return rl_ok_null();
}

rl_result rl_term_move_down(int64_t n) {
    printf("\x1b[%ldB", (long)n);
    return rl_ok_null();
}

rl_result rl_term_move_left(int64_t n) {
    printf("\x1b[%ldD", (long)n);
    return rl_ok_null();
}

rl_result rl_term_move_right(int64_t n) {
    printf("\x1b[%ldC", (long)n);
    return rl_ok_null();
}

rl_result rl_term_next_line(int64_t n) {
    printf("\x1b[%ldE", (long)n);
    return rl_ok_null();
}

rl_result rl_term_prev_line(int64_t n) {
    printf("\x1b[%ldF", (long)n);
    return rl_ok_null();
}

rl_result rl_term_save_cursor(void) {
    printf("\x1b[s");
    return rl_ok_null();
}

rl_result rl_term_restore_cursor(void) {
    printf("\x1b[u");
    return rl_ok_null();
}

rl_result rl_term_hide_cursor(void) {
    printf("\x1b[?25l");
    return rl_ok_null();
}

rl_result rl_term_show_cursor(void) {
    printf("\x1b[?25h");
    return rl_ok_null();
}

rl_result rl_term_get_size(int64_t *out_cols, int64_t *out_rows) {
    struct winsize ws;
    if (ioctl(STDOUT_FILENO, TIOCGWINSZ, &ws) == 0) {
        *out_cols = ws.ws_col;
        *out_rows = ws.ws_row;
    } else {
        *out_cols = 80;
        *out_rows = 24;
    }
    return rl_ok_null();
}

rl_result rl_term_set_size(int64_t cols, int64_t rows) {
    printf("\x1b[8;%ld;%ldt", (long)rows, (long)cols);
    return rl_ok_null();
}

rl_result rl_term_set_title(int64_t ch) {
    printf("\x1b]0;%c\x07", (char)(unsigned char)ch);
    return rl_ok_null();
}

rl_result rl_term_scroll_up(int64_t n) {
    printf("\x1b[%ldS", (long)n);
    return rl_ok_null();
}

rl_result rl_term_scroll_down(int64_t n) {
    printf("\x1b[%ldT", (long)n);
    return rl_ok_null();
}

rl_result rl_term_flush(void) {
    fflush(stdout);
    return rl_ok_null();
}

rl_result rl_term_set_fg(int64_t r, int64_t g, int64_t b) {
    printf("\x1b[38;2;%ld;%ld;%ldm", (long)r, (long)g, (long)b);
    return rl_ok_null();
}

rl_result rl_term_set_bg(int64_t r, int64_t g, int64_t b) {
    printf("\x1b[48;2;%ld;%ld;%ldm", (long)r, (long)g, (long)b);
    return rl_ok_null();
}

rl_result rl_term_reset_color(void) {
    printf("\x1b[0m");
    return rl_ok_null();
}

static int _term_named_color(rl_string name) {
    if (name.len == 3 && memcmp(name.data, "red", 3) == 0) return 31;
    if (name.len == 5 && memcmp(name.data, "green", 5) == 0) return 32;
    if (name.len == 6 && memcmp(name.data, "yellow", 6) == 0) return 33;
    if (name.len == 4 && memcmp(name.data, "blue", 4) == 0) return 34;
    if (name.len == 7 && memcmp(name.data, "magenta", 7) == 0) return 35;
    if (name.len == 4 && memcmp(name.data, "cyan", 4) == 0) return 36;
    if (name.len == 5 && memcmp(name.data, "white", 5) == 0) return 37;
    if (name.len == 5 && memcmp(name.data, "black", 5) == 0) return 30;
    if (name.len == 10 && memcmp(name.data, "dark_black", 10) == 0) return 90;
    if (name.len == 8 && memcmp(name.data, "dark_red", 8) == 0) return 91;
    if (name.len == 10 && memcmp(name.data, "dark_green", 10) == 0) return 92;
    if (name.len == 11 && memcmp(name.data, "dark_yellow", 11) == 0) return 93;
    if (name.len == 9 && memcmp(name.data, "dark_blue", 9) == 0) return 94;
    if (name.len == 12 && memcmp(name.data, "dark_magenta", 12) == 0) return 95;
    if (name.len == 9 && memcmp(name.data, "dark_cyan", 9) == 0) return 96;
    if (name.len == 4 && memcmp(name.data, "grey", 4) == 0) return 90;
    return -1;
}

rl_result rl_term_fg(rl_string name) {
    int code = _term_named_color(name);
    if (code < 0) {
        return rl_err_msg(rl_str_literal("term_fg(): unknown color", 24));
    }
    printf("\x1b[%dm", code);
    return rl_ok_null();
}

rl_result rl_term_bg(rl_string name) {
    int code = _term_named_color(name);
    if (code < 0) {
        return rl_err_msg(rl_str_literal("term_bg(): unknown color", 24));
    }
    printf("\x1b[%dm", code + 10);
    return rl_ok_null();
}

rl_result rl_term_bold(void) { printf("\x1b[1m"); return rl_ok_null(); }
rl_result rl_term_dim(void) { printf("\x1b[2m"); return rl_ok_null(); }
rl_result rl_term_italic(void) { printf("\x1b[3m"); return rl_ok_null(); }
rl_result rl_term_underline(void) { printf("\x1b[4m"); return rl_ok_null(); }
rl_result rl_term_blink(void) { printf("\x1b[5m"); return rl_ok_null(); }
rl_result rl_term_reverse(void) { printf("\x1b[7m"); return rl_ok_null(); }
rl_result rl_term_crossed_out(void) { printf("\x1b[9m"); return rl_ok_null(); }
rl_result rl_term_reset_attr(void) { printf("\x1b[0m"); return rl_ok_null(); }

rl_result rl_term_enable_wrap(void) { printf("\x1b[?7h"); return rl_ok_null(); }
rl_result rl_term_disable_wrap(void) { printf("\x1b[?7l"); return rl_ok_null(); }

rl_result rl_term_begin_sync(void) { printf("\x1b[?2026h"); return rl_ok_null(); }
rl_result rl_term_end_sync(void) { printf("\x1b[?2026l"); return rl_ok_null(); }

rl_result rl_term_enable_mouse(void) { printf("\x1b[?1003h\x1b[?1006h"); return rl_ok_null(); }
rl_result rl_term_disable_mouse(void) { printf("\x1b[?1003l\x1b[?1006l"); return rl_ok_null(); }

void rl_term_print_inline(rl_result v) {
    switch (v.tag) {
        case RL_TAG_I64: printf("%ld", (long)v.data.i64); break;
        case RL_TAG_F64: printf("%g", v.data.f64); break;
        case RL_TAG_BOOL: printf("%s", v.data.boolean ? "true" : "false"); break;
        case RL_TAG_CHAR: printf("%c", (char)(unsigned char)v.data.i64); break;
        case RL_TAG_STR: printf("%.*s", (int)v.data.str.len, v.data.str.data); break;
        case RL_TAG_NULL: printf("null"); break;
        default: printf("<value>"); break;
    }
}

rl_array rl_term_read_key(void) {
    char buf[32];
    uint64_t total = 0;
    // blocking read of one byte
    uint8_t c;
    if (read(STDIN_FILENO, &c, 1) != 1) {
        rl_string empty = rl_str_literal("", 0);
        rl_array arr = { .data = NULL, .len = 0, .cap = 0, .elem_size = sizeof(rl_string), .type_tag = RL_TAG_STR };
        return arr;
    }
    buf[total++] = c;
    // if escape, read more
    if (c == 27) {
        uint8_t next;
        // read with timeout using select
        fd_set fds;
        struct timeval tv = { .tv_sec = 0, .tv_usec = 100000 };
        FD_ZERO(&fds);
        FD_SET(STDIN_FILENO, &fds);
        while (select(STDIN_FILENO + 1, &fds, NULL, NULL, &tv) > 0) {
            if (read(STDIN_FILENO, &next, 1) != 1) break;
            buf[total++] = next;
            if (total >= sizeof(buf) - 1) break;
            FD_ZERO(&fds);
            FD_SET(STDIN_FILENO, &fds);
            tv.tv_sec = 0;
            tv.tv_usec = 10000;
        }
    }
    rl_string s = rl_str_literal(buf, total);
    rl_string *sarr = malloc(sizeof(rl_string));
    sarr[0] = s;
    rl_array arr = { .data = sarr, .len = 1, .cap = 1, .elem_size = sizeof(rl_string), .type_tag = RL_TAG_STR };
    return arr;
}

bool rl_term_poll(int64_t ms) {
    fd_set fds;
    struct timeval tv = { .tv_sec = (long)(ms / 1000), .tv_usec = (long)((ms % 1000) * 1000) };
    FD_ZERO(&fds);
    FD_SET(STDIN_FILENO, &fds);
    return select(STDIN_FILENO + 1, &fds, NULL, NULL, &tv) > 0;
}

// ---- result unwrap (with error checking) ----

int64_t rl_result_unwrap_i64(rl_result r) {
    if (!r.is_ok) {
        fprintf(stderr, "error: unwrap called on err value\n");
        abort();
    }
    return r.data.i64;
}

double rl_result_unwrap_f64(rl_result r) {
    if (!r.is_ok) {
        fprintf(stderr, "error: unwrap called on err value\n");
        abort();
    }
    return r.data.f64;
}

bool rl_result_unwrap_bool(rl_result r) {
    if (!r.is_ok) {
        fprintf(stderr, "error: unwrap called on err value\n");
        abort();
    }
    return r.data.boolean;
}

rl_string rl_result_unwrap_str(rl_result r) {
    if (!r.is_ok) {
        fprintf(stderr, "error: unwrap called on err value\n");
        abort();
    }
    return r.data.str;
}

// ---- math ----

rl_result rl_math_abs(rl_result x) {
    switch (x.tag) {
        case RL_TAG_F64: return rl_ok_f64(fabs(x.data.f64));
        default: return rl_ok_i64(llabs(x.data.i64));
    }
}

rl_result rl_math_pow(rl_result base, rl_result exp) {
    if (base.tag == RL_TAG_I64 && exp.tag == RL_TAG_I64) {
        int64_t b = base.data.i64;
        int64_t e = exp.data.i64;
        if (e < 0) {
            return rl_ok_f64(pow((double)b, (double)e));
        }
        int64_t result = 1;
        while (e > 0) {
            if (e & 1) result *= b;
            e >>= 1;
            b *= b;
        }
        return rl_ok_i64(result);
    }
    double b_val = (base.tag == RL_TAG_F64) ? base.data.f64 : (double)base.data.i64;
    double e_val = (exp.tag == RL_TAG_F64) ? exp.data.f64 : (double)exp.data.i64;
    return rl_ok_f64(pow(b_val, e_val));
}

// ---- type checks ----

rl_result rl_is_bool(rl_result x) { return rl_ok_bool(x.tag == RL_TAG_BOOL); }
rl_result rl_is_int(rl_result x) { return rl_ok_bool(x.tag == RL_TAG_I64); }
rl_result rl_is_float(rl_result x) { return rl_ok_bool(x.tag == RL_TAG_F64); }
rl_result rl_is_string(rl_result x) { return rl_ok_bool(x.tag == RL_TAG_STR); }
rl_result rl_is_null(rl_result x) { return rl_ok_bool(x.tag == RL_TAG_NULL); }
rl_result rl_is_char(rl_result x) { return rl_ok_bool(x.tag == RL_TAG_CHAR); }
rl_result rl_is_byte(rl_result x) { return rl_ok_bool(x.tag == RL_TAG_I64); }
rl_result rl_is_error(rl_result x) { return rl_ok_bool(!x.is_ok); }

rl_never rl_never_fn(void) {
    fprintf(stderr, "error: reached unreachable code\n");
    abort();
}
