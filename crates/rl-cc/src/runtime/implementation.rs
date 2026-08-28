pub const RUNTIME_C: &str = r#"#include "rl_runtime.h"

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

void rl_print_int64(int64_t v) { printf("%ld", v); }
void rl_print_float64(double v) { printf("%g", v); }
void rl_print_bool(bool v) { printf(v ? "true" : "false"); }
void rl_print_char(char v) { printf("%c", v); }
void rl_print_str(rl_string v) { printf("%.*s", (int)v.len, v.data); }
void rl_print_ptr(void *v) { printf("<ptr:%p>", v); }

void rl_println_int64(int64_t v) { printf("%ld\n", v); }
void rl_println_float64(double v) { printf("%g\n", v); }
void rl_println_bool(bool v) { printf("%s\n", v ? "true" : "false"); }
void rl_println_char(char v) { printf("%c\n", v); }
void rl_println_str(rl_string v) { printf("%.*s\n", (int)v.len, v.data); }
void rl_println_ptr(void *v) { printf("<ptr:%p>\n", v); }

rl_never rl_never_fn(void) {
    fprintf(stderr, "error: reached unreachable code\n");
    abort();
}
"#;
