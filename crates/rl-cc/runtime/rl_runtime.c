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

void rl_print_rl_array(rl_array v) {
    printf("[");
    if (v.elem_size == sizeof(int64_t)) {
        int64_t *data = (int64_t *)v.data;
        for (uint64_t i = 0; i < v.len; i++) {
            if (i > 0) printf(", ");
            printf("%ld", (long)data[i]);
        }
    } else if (v.elem_size == sizeof(rl_string)) {
        rl_string *data = (rl_string *)v.data;
        for (uint64_t i = 0; i < v.len; i++) {
            if (i > 0) printf(", ");
            printf("\"%.*s\"", (int)data[i].len, data[i].data);
        }
    } else {
        printf("...");
    }
    printf("]");
}

void rl_println_rl_array(rl_array v) {
    rl_print_rl_array(v);
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

int64_t rl_fs_mkdir(rl_string path) {
    char buf[path.len + 1];
    memcpy(buf, path.data, path.len);
    buf[path.len] = '\0';
    return mkdir(buf, 0755);
}

// ---- string ----

static uint64_t rl_str_utf8_len(const char *s, uint64_t byte_len) {
    uint64_t count = 0;
    for (uint64_t i = 0; i < byte_len; ) {
        unsigned char c = (unsigned char)s[i];
        if (c < 0x80) { i += 1; }
        else if (c < 0xE0) { i += 2; }
        else if (c < 0xF0) { i += 3; }
        else { i += 4; }
        count++;
    }
    return count;
}

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
    // count occurrences
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
    uint64_t new_len = s.len + count * (to.len > from.len ? to.len - from.len : from.len - to.len)
                       + count * (to.len > from.len ? to.len - from.len : 0);
    // more precise: new_len = s.len - count * from.len + count * to.len
    new_len = s.len - count * from.len + count * to.len;
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

// ---- string (continued) ----

rl_array rl_str_bytes(rl_string s) {
    int64_t *buf = malloc(s.len * sizeof(int64_t));
    for (uint64_t i = 0; i < s.len; i++) {
        buf[i] = (int64_t)(unsigned char)s.data[i];
    }
    return rl_arr_from_vals(buf, s.len, sizeof(int64_t));
}

rl_array rl_str_chars(rl_string s) {
    uint64_t cap = 16;
    int64_t *buf = malloc(cap * sizeof(int64_t));
    uint64_t count = 0;
    uint64_t i = 0;
    while (i < s.len) {
        unsigned char c = (unsigned char)s.data[i];
        uint64_t char_len;
        int64_t codepoint;
        if (c < 0x80) {
            char_len = 1;
            codepoint = c;
        } else if (c < 0xE0) {
            char_len = 2;
            codepoint = c & 0x1F;
        } else if (c < 0xF0) {
            char_len = 3;
            codepoint = c & 0x0F;
        } else {
            char_len = 4;
            codepoint = c & 0x07;
        }
        for (uint64_t j = 1; j < char_len && i + j < s.len; j++) {
            codepoint = (codepoint << 6) | ((unsigned char)s.data[i + j] & 0x3F);
        }
        if (count >= cap) {
            cap *= 2;
            buf = realloc(buf, cap * sizeof(int64_t));
        }
        buf[count++] = codepoint;
        i += char_len;
    }
    return rl_arr_from_vals(buf, count, sizeof(int64_t));
}

int64_t rl_str_char_at(rl_string s, int64_t index) {
    if (index < 0) return -1;
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
        if (char_idx == index) return codepoint;
        pos += char_len;
        char_idx++;
    }
    return -1;
}

rl_string rl_str_join(rl_array arr, rl_string delim) {
    if (arr.len == 0) {
        rl_string result = { .data = "", .len = 0, .rc = 1 };
        return result;
    }
    // estimate: each element ~8 bytes avg + delim between
    uint64_t cap = arr.len * 8 + arr.len * delim.len + 1;
    char *buf = malloc(cap);
    uint64_t w = 0;
    int64_t *elems = (int64_t *)arr.data;
    for (uint64_t i = 0; i < arr.len; i++) {
        if (i > 0) {
            memcpy(buf + w, delim.data, delim.len);
            w += delim.len;
        }
        // convert each int64 to string
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
        rl_array arr;
        arr.data = NULL;
        arr.len = 0;
        arr.cap = 0;
        arr.elem_size = sizeof(rl_string);
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
    fprintf(stderr, "[dbg] %f (float)\n", v);
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
    // find last '.'
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

int64_t rl_path_is_dir(rl_string path) {
    char buf[path.len + 1];
    memcpy(buf, path.data, path.len);
    buf[path.len] = '\0';
    struct stat st;
    if (stat(buf, &st) != 0) return 0;
    return S_ISDIR(st.st_mode) ? 1 : 0;
}

int64_t rl_path_is_file(rl_string path) {
    char buf[path.len + 1];
    memcpy(buf, path.data, path.len);
    buf[path.len] = '\0';
    struct stat st;
    if (stat(buf, &st) != 0) return 0;
    return S_ISREG(st.st_mode) ? 1 : 0;
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
    // create each component
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
    // simple: try rmdir (non-recursive for safety)
    return rmdir(buf);
}

rl_array rl_fs_list_dir(rl_string path) {
    char buf[path.len + 1];
    memcpy(buf, path.data, path.len);
    buf[path.len] = '\0';
    DIR *d = opendir(buf);
    if (!d) {
        rl_array arr;
        arr.data = NULL;
        arr.len = 0;
        arr.cap = 0;
        arr.elem_size = sizeof(rl_string);
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
        uint64_t len = strlen(ent->d_name);
        char *name = malloc(len + 1);
        memcpy(name, ent->d_name, len + 1);
        entries[count] = (rl_string){ .data = name, .len = len, .rc = 1 };
        count++;
    }
    closedir(d);
    rl_array arr;
    arr.data = entries;
    arr.len = count;
    arr.cap = cap;
    arr.elem_size = sizeof(rl_string);
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
    // split by newline
    rl_string nl = { .data = "\n", .len = 1, .rc = 1 };
    return rl_str_split(output, nl);
}

rl_string rl_process_with_exec(rl_string env, rl_string cmd) {
    // combine env and cmd
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

rl_array rl_process_args(void) {
    rl_array arr;
    arr.data = NULL;
    arr.len = 0;
    arr.cap = 0;
    arr.elem_size = sizeof(rl_string);
    return arr;
}

// ---- time ----

rl_string rl_time_format_time(int64_t timestamp, rl_string pattern) {
    time_t t = (time_t)timestamp;
    struct tm *tm = localtime(&t);
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
    struct tm *tm = localtime(&t);
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
    struct tm *tm = localtime(&t);
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
    struct tm *tm = localtime(&t);
    int64_t parts[7] = {
        tm->tm_year + 1900,
        tm->tm_mon + 1,
        tm->tm_mday,
        tm->tm_hour,
        tm->tm_min,
        tm->tm_sec,
        tm->tm_wday
    };
    return rl_arr_from_vals(parts, 7, sizeof(int64_t));
}

// ---- io ----

rl_string rl_io_read_file(rl_string path) {
    char buf[path.len + 1];
    memcpy(buf, path.data, path.len);
    buf[path.len] = '\0';
    FILE *f = fopen(buf, "rb");
    if (!f) {
        rl_string result = { .data = "", .len = 0, .rc = 1 };
        return result;
    }
    fseek(f, 0, SEEK_END);
    long size = ftell(f);
    fseek(f, 0, SEEK_SET);
    if (size <= 0) {
        fclose(f);
        rl_string result = { .data = "", .len = 0, .rc = 1 };
        return result;
    }
    char *out = malloc((uint64_t)size);
    size_t n = fread(out, 1, (uint64_t)size, f);
    fclose(f);
    rl_string result = { .data = out, .len = (uint64_t)n, .rc = 1 };
    return result;
}

rl_array rl_io_read_lines(rl_string path) {
    rl_string content = rl_io_read_file(path);
    rl_string nl = { .data = "\n", .len = 1, .rc = 1 };
    return rl_str_split(content, nl);
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

int64_t rl_io_write_file(rl_string path, rl_string content) {
    char buf[path.len + 1];
    memcpy(buf, path.data, path.len);
    buf[path.len] = '\0';
    FILE *f = fopen(buf, "wb");
    if (!f) return -1;
    fwrite(content.data, 1, content.len, f);
    fclose(f);
    return 0;
}

int64_t rl_io_append_file(rl_string path, rl_string content) {
    char buf[path.len + 1];
    memcpy(buf, path.data, path.len);
    buf[path.len] = '\0';
    FILE *f = fopen(buf, "ab");
    if (!f) return -1;
    fwrite(content.data, 1, content.len, f);
    fclose(f);
    return 0;
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

// ---- collections (thin wrappers) ----

int64_t rl_set_add_i64(rl_set *s, int64_t value) {
    rl_set_add(s, value);
    return 1;
}

int64_t rl_set_remove_i64(rl_set *s, int64_t value) {
    rl_set_remove(s, value);
    return 1;
}

int64_t rl_set_contains_i64(rl_set s, int64_t value) {
    return rl_set_contains(s, value) ? 1 : 0;
}

rl_array rl_set_to_array_i64(rl_set s) {
    int64_t *buf = malloc(s.len * sizeof(int64_t));
    memcpy(buf, s.data, s.len * sizeof(int64_t));
    return rl_arr_from_vals(buf, s.len, sizeof(int64_t));
}

int64_t rl_map_contains_s(rl_map m, rl_string key) {
    char buf[key.len + 1];
    memcpy(buf, key.data, key.len);
    buf[key.len] = '\0';
    return rl_map_contains(m, buf) ? 1 : 0;
}

int64_t rl_map_remove_s(rl_map *m, rl_string key) {
    char buf[key.len + 1];
    memcpy(buf, key.data, key.len);
    buf[key.len] = '\0';
    rl_map_remove(m, buf);
    return 1;
}

int64_t rl_map_remove_val(rl_map m, rl_string key) {
    // non-pointer version for codegen compatibility
    return rl_map_remove_s(&m, key);
}

int64_t rl_map_get_s(rl_map m, rl_string key) {
    char buf[key.len + 1];
    memcpy(buf, key.data, key.len);
    buf[key.len] = '\0';
    return rl_map_get(m, buf);
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
    int64_t *buf = malloc(m.len * sizeof(int64_t));
    for (uint64_t i = 0; i < m.len; i++) {
        buf[i] = m.entries[i].value;
    }
    return rl_arr_from_vals(buf, m.len, sizeof(int64_t));
}

rl_map rl_map_merge_s(rl_map a, rl_map b) {
    // merge b into a copy of a
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
    int64_t *buf = malloc(m.len * sizeof(int64_t));
    for (uint64_t i = 0; i < m.len; i++) {
        buf[i] = m.entries[i].value;
    }
    return rl_arr_from_vals(buf, m.len, sizeof(int64_t));
}

// ---- array (extended) ----

rl_array rl_arr_push_i64(rl_array a, int64_t v) {
    uint64_t new_len = a.len + 1;
    int64_t *buf = malloc(new_len * sizeof(int64_t));
    if (a.data) memcpy(buf, a.data, a.len * sizeof(int64_t));
    buf[a.len] = v;
    return rl_arr_from_vals(buf, new_len, sizeof(int64_t));
}

int64_t rl_arr_pop_i64(rl_array a) {
    if (a.len == 0) return 0;
    int64_t *data = (int64_t *)a.data;
    return data[a.len - 1];
}

rl_array rl_arr_insert_i64(rl_array a, int64_t idx, int64_t v) {
    if (idx < 0) idx = 0;
    if ((uint64_t)idx > a.len) idx = (int64_t)a.len;
    uint64_t new_len = a.len + 1;
    int64_t *buf = malloc(new_len * sizeof(int64_t));
    int64_t *src = (int64_t *)a.data;
    if (idx > 0 && src) memcpy(buf, src, (uint64_t)idx * sizeof(int64_t));
    buf[idx] = v;
    if (src && (uint64_t)idx < a.len) memcpy(buf + idx + 1, src + idx, (a.len - (uint64_t)idx) * sizeof(int64_t));
    return rl_arr_from_vals(buf, new_len, sizeof(int64_t));
}

rl_array rl_arr_remove_i64(rl_array a, int64_t idx) {
    if (idx < 0 || (uint64_t)idx >= a.len) return a;
    int64_t *src = (int64_t *)a.data;
    uint64_t new_len = a.len - 1;
    int64_t *buf = malloc(new_len * sizeof(int64_t));
    if (idx > 0) memcpy(buf, src, (uint64_t)idx * sizeof(int64_t));
    if ((uint64_t)idx < a.len - 1) memcpy(buf + idx, src + idx + 1, (a.len - (uint64_t)idx - 1) * sizeof(int64_t));
    return rl_arr_from_vals(buf, new_len, sizeof(int64_t));
}

rl_array rl_arr_reverse_i64(rl_array a) {
    int64_t *src = (int64_t *)a.data;
    int64_t *buf = malloc(a.len * sizeof(int64_t));
    for (uint64_t i = 0; i < a.len; i++) {
        buf[i] = src[a.len - 1 - i];
    }
    return rl_arr_from_vals(buf, a.len, sizeof(int64_t));
}

rl_array rl_arr_concat_i64(rl_array a, rl_array b) {
    uint64_t new_len = a.len + b.len;
    int64_t *buf = malloc(new_len * sizeof(int64_t));
    if (a.data) memcpy(buf, a.data, a.len * sizeof(int64_t));
    if (b.data) memcpy(buf + a.len, b.data, b.len * sizeof(int64_t));
    return rl_arr_from_vals(buf, new_len, sizeof(int64_t));
}

int64_t rl_arr_first_i64(rl_array a) {
    if (a.len == 0) return 0;
    return ((int64_t *)a.data)[0];
}

int64_t rl_arr_last_i64(rl_array a) {
    if (a.len == 0) return 0;
    return ((int64_t *)a.data)[a.len - 1];
}

rl_array rl_arr_unique_i64(rl_array a) {
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

rl_array rl_arr_slice_i64(rl_array a, int64_t start, int64_t end) {
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

int64_t rl_arr_contains_i64(rl_array a, int64_t v) {
    int64_t *src = (int64_t *)a.data;
    for (uint64_t i = 0; i < a.len; i++) {
        if (src[i] == v) return 1;
    }
    return 0;
}

int64_t rl_arr_index_of_i64(rl_array a, int64_t v) {
    int64_t *src = (int64_t *)a.data;
    for (uint64_t i = 0; i < a.len; i++) {
        if (src[i] == v) return (int64_t)i;
    }
    return -1;
}

rl_array rl_arr_fill_i64(int64_t v, int64_t count) {
    if (count <= 0) return rl_arr_from_vals(NULL, 0, sizeof(int64_t));
    int64_t *buf = malloc((uint64_t)count * sizeof(int64_t));
    for (int64_t i = 0; i < count; i++) buf[i] = v;
    return rl_arr_from_vals(buf, (uint64_t)count, sizeof(int64_t));
}

rl_array rl_arr_range_i64(int64_t start, int64_t end, int64_t step) {
    if (step == 0) step = 1;
    uint64_t cap = 16;
    int64_t *buf = malloc(cap * sizeof(int64_t));
    uint64_t count = 0;
    if (step > 0) {
        for (int64_t i = start; i < end; i += step) {
            if (count >= cap) { cap *= 2; buf = realloc(buf, cap * sizeof(int64_t)); }
            buf[count++] = i;
        }
    } else {
        for (int64_t i = start; i > end; i += step) {
            if (count >= cap) { cap *= 2; buf = realloc(buf, cap * sizeof(int64_t)); }
            buf[count++] = i;
        }
    }
    return rl_arr_from_vals(buf, count, sizeof(int64_t));
}

int64_t rl_arr_sum_i64(rl_array a) {
    int64_t *src = (int64_t *)a.data;
    int64_t sum = 0;
    for (uint64_t i = 0; i < a.len; i++) sum += src[i];
    return sum;
}

int64_t rl_arr_product_i64(rl_array a) {
    int64_t *src = (int64_t *)a.data;
    int64_t prod = 1;
    for (uint64_t i = 0; i < a.len; i++) prod *= src[i];
    return prod;
}

int64_t rl_arr_max_i64(rl_array a) {
    if (a.len == 0) return 0;
    int64_t *src = (int64_t *)a.data;
    int64_t max = src[0];
    for (uint64_t i = 1; i < a.len; i++) {
        if (src[i] > max) max = src[i];
    }
    return max;
}

int64_t rl_arr_min_i64(rl_array a) {
    if (a.len == 0) return 0;
    int64_t *src = (int64_t *)a.data;
    int64_t min = src[0];
    for (uint64_t i = 1; i < a.len; i++) {
        if (src[i] < min) min = src[i];
    }
    return min;
}

static int rl_arr_cmp_i64(const void *a, const void *b) {
    int64_t va = *(const int64_t *)a;
    int64_t vb = *(const int64_t *)b;
    return (va > vb) - (va < vb);
}

rl_array rl_arr_sort_i64(rl_array a) {
    int64_t *buf = malloc(a.len * sizeof(int64_t));
    if (a.data) memcpy(buf, a.data, a.len * sizeof(int64_t));
    qsort(buf, a.len, sizeof(int64_t), rl_arr_cmp_i64);
    return rl_arr_from_vals(buf, a.len, sizeof(int64_t));
}

rl_array rl_arr_flatten_i64(rl_array a) {
    // a is array of rl_array (each elem_size == sizeof(rl_array))
    // but since we use int64_t arrays, flatten is a no-op for simple arrays
    // return a copy
    int64_t *buf = malloc(a.len * sizeof(int64_t));
    if (a.data) memcpy(buf, a.data, a.len * sizeof(int64_t));
    return rl_arr_from_vals(buf, a.len, sizeof(int64_t));
}

rl_never rl_never_fn(void) {
    fprintf(stderr, "error: reached unreachable code\n");
    abort();
}
