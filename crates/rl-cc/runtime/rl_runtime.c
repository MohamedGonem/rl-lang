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

// ---- result type ----

rl_result rl_ok(int64_t value) {
    rl_result r = { .is_ok = true, .data.ok_value = value, .err_code = 0 };
    return r;
}

rl_result rl_err(int64_t value) {
    rl_result r = { .is_ok = false, .data.err_value = value, .err_code = 0 };
    return r;
}

rl_result rl_error(int64_t value) {
    rl_result r = { .is_ok = false, .data.err_value = value, .err_code = -1 };
    return r;
}

// ---- array type ----

rl_array rl_arr_from_vals(const void *vals, uint64_t count, int32_t elem_size) {
    rl_array arr;
    arr.len = count;
    arr.cap = count;
    arr.elem_size = elem_size;
    if (count == 0) {
        arr.data = NULL;
    } else {
        arr.data = malloc(count * elem_size);
        memcpy(arr.data, vals, count * elem_size);
    }
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

void rl_map_set(rl_map *m, const char *key, int64_t val) {
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

int64_t rl_map_get(rl_map m, const char *key) {
    for (uint64_t i = 0; i < m.len; i++) {
        if (strcmp(m.entries[i].key, key) == 0) {
            return m.entries[i].value;
        }
    }
    fprintf(stderr, "error: key '%s' not found in map\n", key);
    abort();
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
    s->data = realloc(s->data, new_cap * sizeof(int64_t));
    s->cap = new_cap;
}

void rl_set_add(rl_set *s, int64_t val) {
    for (uint64_t i = 0; i < s->len; i++) {
        if (s->data[i] == val) return;
    }
    rl_set_grow(s, s->len + 1);
    s->data[s->len++] = val;
}

bool rl_set_contains(rl_set s, int64_t val) {
    for (uint64_t i = 0; i < s.len; i++) {
        if (s.data[i] == val) return true;
    }
    return false;
}

uint64_t rl_set_len(rl_set s) { return s.len; }

void rl_set_remove(rl_set *s, int64_t val) {
    for (uint64_t i = 0; i < s->len; i++) {
        if (s->data[i] == val) {
            s->data[i] = s->data[s->len - 1];
            s->len--;
            return;
        }
    }
}

// ---- print functions ----
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

void rl_print_result(rl_result v) {
    if (v.is_ok) {
        printf("ok(%ld)", (long)v.data.ok_value);
    } else {
        printf("err(%ld)", (long)v.data.err_value);
    }
}
void rl_println_result(rl_result v) {
    rl_print_result(v);
    printf("\n");
}

void rl_print_rl_map(rl_map v) {
    printf("{");
    for (uint64_t i = 0; i < v.len; i++) {
        if (i > 0) printf(", ");
        printf("\"%s\": %ld", v.entries[i].key, (long)v.entries[i].value);
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
        printf("%ld", (long)v.data[i]);
    }
    printf("}");
}

void rl_println_rl_set(rl_set v) {
    rl_print_rl_set(v);
    printf("\n");
}

rl_never rl_never_fn(void) {
    fprintf(stderr, "error: reached unreachable code\n");
    abort();
}
