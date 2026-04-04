/* Q++ runtime - compiled to runtime.o / runtime.obj and linked with LLVM output */

#include "runtime.h"

#ifdef _WIN32
#include <time.h>
#define strdup _strdup
#else
#include <time.h>
#endif

/* ========== STR ========== */
char* ql_str_trim(const char* s) {
    if (!s) return (char*)"";
    while (*s == ' ' || *s == '\t' || *s == '\n' || *s == '\r') s++;
    const char* end = s;
    while (*end) end++;
    while (end > s && (end[-1] == ' ' || end[-1] == '\t' || end[-1] == '\n' || end[-1] == '\r')) end--;
    size_t n = end - s;
    char* r = (char*)malloc(n + 1);
    memcpy(r, s, n);
    r[n] = 0;
    return r;
}
char* ql_str_concat(const char* a, const char* b) {
    if (!a) a = "";
    if (!b) b = "";
    size_t la = strlen(a), lb = strlen(b);
    char* r = (char*)malloc(la + lb + 1);
    memcpy(r, a, la + 1);
    strcat(r, b);
    return r;
}

/* ========== HASH ========== */
uint64_t ql_hash_fnv1a(const void* ptr, size_t len) {
    const uint8_t* p = (const uint8_t*)ptr;
    uint64_t h = 14695981039346656037ULL;
    for (size_t i = 0; i < len; i++) {
        h ^= (uint64_t)p[i];
        h *= 1099511628211ULL;
    }
    return h;
}
uint64_t ql_hash_djb2(const void* ptr, size_t len) {
    const uint8_t* p = (const uint8_t*)ptr;
    uint64_t h = 5381;
    for (size_t i = 0; i < len; i++) {
        h = ((h << 5) + h) + (uint64_t)p[i];
    }
    return h;
}

/* ========== TERM ========== */
void ql_term_reset(void) { printf("\033[0m"); }
void ql_term_red(void) { printf("\033[31m"); }
void ql_term_green(void) { printf("\033[32m"); }
void ql_term_yellow(void) { printf("\033[33m"); }
void ql_term_blue(void) { printf("\033[34m"); }
void ql_term_magenta(void) { printf("\033[35m"); }
void ql_term_cyan(void) { printf("\033[36m"); }
void ql_term_bold(void) { printf("\033[1m"); }
void ql_term_clear_screen(void) { printf("\033[2J\033[H"); }
void ql_term_cursor_hide(void) { printf("\033[?25l"); }
void ql_term_cursor_show(void) { printf("\033[?25h"); }

/* ========== VEC ========== */
void* ql_vec_ptr_new(void) {
    ql_vec_ptr_t* v = (ql_vec_ptr_t*)malloc(sizeof(ql_vec_ptr_t));
    v->data = 0; v->len = 0; v->cap = 0;
    return v;
}
void ql_vec_ptr_push(void* vp, void* ptr) {
    ql_vec_ptr_t* v = (ql_vec_ptr_t*)vp;
    if (v->len >= v->cap) {
        size_t ncap = v->cap ? v->cap * 2 : 16;
        v->data = (void**)realloc(v->data, ncap * sizeof(void*));
        v->cap = ncap;
    }
    v->data[v->len++] = ptr;
}
void* ql_vec_ptr_get(void* vp, size_t i) {
    ql_vec_ptr_t* v = (ql_vec_ptr_t*)vp;
    return (i < v->len) ? v->data[i] : 0;
}
size_t ql_vec_ptr_len(void* vp) { return ((ql_vec_ptr_t*)vp)->len; }
void ql_vec_ptr_clear(void* vp) { ((ql_vec_ptr_t*)vp)->len = 0; }
void ql_vec_ptr_free(void* vp) {
    ql_vec_ptr_t* v = (ql_vec_ptr_t*)vp;
    free(v->data);
    free(v);
}
void* ql_vec_u8_new(void) {
    ql_vec_u8_t* v = (ql_vec_u8_t*)malloc(sizeof(ql_vec_u8_t));
    v->data = 0; v->len = 0; v->cap = 0;
    return v;
}
void ql_vec_u8_push(void* vp, unsigned char b) {
    ql_vec_u8_t* v = (ql_vec_u8_t*)vp;
    if (v->len >= v->cap) {
        size_t ncap = v->cap ? v->cap * 2 : 64;
        v->data = (char*)realloc(v->data, ncap);
        v->cap = ncap;
    }
    v->data[v->len++] = (char)b;
}
void ql_vec_u8_append(void* vp, const char* s) {
    if (!s) return;
    size_t n = strlen(s);
    ql_vec_u8_t* v = (ql_vec_u8_t*)vp;
    while (v->len + n >= v->cap) {
        size_t ncap = v->cap ? v->cap * 2 : 64;
        if (ncap < v->len + n + 1) ncap = v->len + n + 1;
        v->data = (char*)realloc(v->data, ncap);
        v->cap = ncap;
    }
    memcpy(v->data + v->len, s, n);
    v->len += n;
}
size_t ql_vec_u8_len(void* vp) { return ((ql_vec_u8_t*)vp)->len; }
char* ql_vec_u8_to_str(void* vp) {
    ql_vec_u8_t* v = (ql_vec_u8_t*)vp;
    char* r = (char*)malloc(v->len + 1);
    memcpy(r, v->data, v->len);
    r[v->len] = 0;
    return r;
}
void ql_vec_u8_clear(void* vp) { ((ql_vec_u8_t*)vp)->len = 0; }
void ql_vec_u8_free(void* vp) {
    ql_vec_u8_t* v = (ql_vec_u8_t*)vp;
    free(v->data);
    free(v);
}

/* ========== MAP ========== */
void* ql_map_str_ptr_new(void) { return ql_vec_ptr_new(); }
void ql_map_str_ptr_put(void* mp, const char* key, void* value) {
    void** vp = (void**)mp;
    ql_vec_ptr_t* v = (ql_vec_ptr_t*)vp;
    size_t i;
    for (i = 0; i < v->len; i++) {
        ql_map_pair_t* p = (ql_map_pair_t*)v->data[i];
        if (p && strcmp(p->key, key) == 0) {
            free(p->key);
            p->value = value;
            return;
        }
    }
    ql_map_pair_t* p = (ql_map_pair_t*)malloc(sizeof(ql_map_pair_t));
    p->key = key ? strdup(key) : 0;
    p->value = value;
    ql_vec_ptr_push(mp, p);
}
void* ql_map_str_ptr_get(void* mp, const char* key) {
    ql_vec_ptr_t* v = (ql_vec_ptr_t*)mp;
    size_t i;
    for (i = 0; i < v->len; i++) {
        ql_map_pair_t* p = (ql_map_pair_t*)v->data[i];
        if (p && p->key && key && strcmp(p->key, key) == 0)
            return p->value;
    }
    return 0;
}
int ql_map_str_ptr_has(void* mp, const char* key) {
    return ql_map_str_ptr_get(mp, key) != 0;
}
size_t ql_map_str_ptr_len(void* mp) {
    return ql_vec_ptr_len(mp);
}
void ql_map_str_ptr_free(void* mp) {
    ql_vec_ptr_t* v = (ql_vec_ptr_t*)mp;
    size_t i;
    for (i = 0; i < v->len; i++) {
        ql_map_pair_t* p = (ql_map_pair_t*)v->data[i];
        if (p) { free(p->key); free(p); }
    }
    ql_vec_ptr_free(mp);
}

/* ========== FMT ========== */
int ql_fmt_sprintf_s(char* buf, size_t size, const char* fmt, const char* s) {
    return snprintf(buf, size, fmt, s ? s : "");
}
int ql_fmt_sprintf_ii(char* buf, size_t size, const char* fmt, long a, long b) {
    return snprintf(buf, size, fmt, a, b);
}
int ql_fmt_sprintf_si(char* buf, size_t size, const char* fmt, const char* s, long a) {
    return snprintf(buf, size, fmt, s ? s : "", a);
}
int ql_fmt_sprintf_ss(char* buf, size_t size, const char* fmt, const char* a, const char* b) {
    return snprintf(buf, size, fmt, a ? a : "", b ? b : "");
}
char* ql_fmt_alloc_i(const char* fmt, long a) {
    char buf[64];
    int n = snprintf(buf, sizeof(buf), fmt, a);
    char* r = (char*)malloc((size_t)n + 1);
    memcpy(r, buf, (size_t)n + 1);
    return r;
}
char* ql_fmt_alloc_s(const char* fmt, const char* s) {
    size_t n = strlen(s ? s : "") + 64;
    char* r = (char*)malloc(n);
    snprintf(r, n, fmt, s ? s : "");
    return r;
}
char* ql_fmt_alloc_si(const char* fmt, const char* s, long a) {
    size_t n = strlen(s ? s : "") + 64;
    char* r = (char*)malloc(n);
    snprintf(r, n, fmt, s ? s : "", a);
    return r;
}

/* ========== LEX ========== */
void* ql_token_create(int ty, size_t line, size_t col, const char* str_val, long int_val) {
    ql_token_t* t = (ql_token_t*)malloc(sizeof(ql_token_t));
    t->ty = ty; t->line = line; t->col = col;
    t->str_val = str_val ? strdup(str_val) : 0;
    t->int_val = int_val;
    return t;
}
int ql_token_ty(void* t) { return ((ql_token_t*)t)->ty; }
size_t ql_token_line(void* t) { return ((ql_token_t*)t)->line; }
size_t ql_token_col(void* t) { return ((ql_token_t*)t)->col; }
char* ql_token_str(void* t) { return ((ql_token_t*)t)->str_val; }
long ql_token_int(void* t) { return ((ql_token_t*)t)->int_val; }
void ql_token_free(void* t) {
    ql_token_t* tok = (ql_token_t*)t;
    free(tok->str_val);
    free(tok);
}
int ql_str_at(const char* s, size_t i) {
    if (!s || i >= strlen(s)) return -1;
    return (unsigned char)s[i];
}
char* ql_str_sub(const char* s, size_t start, size_t end) {
    if (!s || start >= end || end > strlen(s)) return strdup("");
    size_t n = end - start;
    char* r = (char*)malloc(n + 1);
    memcpy(r, s + start, n);
    r[n] = 0;
    return r;
}
void* ql_usize_to_ptr(size_t u) { return (void*)(uintptr_t)u; }
size_t ql_ptr_to_usize(void* p) { return (size_t)(uintptr_t)p; }

/* ========== AST ========== */
void* ql_ast_expr_alloc(void) {
    return malloc(sizeof(ast_Expr_t));
}
void ql_ast_expr_set_tag(void* p, int tag) { ((ast_Expr_t*)p)->tag = tag; }
void ql_ast_expr_set_int(void* p, long val) { ((ast_Expr_t*)p)->int_val = val; }
void ql_ast_expr_set_str(void* p, char* s) { ((ast_Expr_t*)p)->str_val = s; }
void ql_ast_expr_set_left(void* p, void* left) { ((ast_Expr_t*)p)->left = left; }
void ql_ast_expr_set_right(void* p, void* right) { ((ast_Expr_t*)p)->right = right; }
void ql_ast_expr_set_args(void* p, void* args) { ((ast_Expr_t*)p)->args = args; }
int ql_ast_expr_tag(void* p) { return ((ast_Expr_t*)p)->tag; }
long ql_ast_expr_int(void* p) { return ((ast_Expr_t*)p)->int_val; }
char* ql_ast_expr_str(void* p) { return ((ast_Expr_t*)p)->str_val; }
void* ql_ast_expr_left(void* p) { return ((ast_Expr_t*)p)->left; }
void* ql_ast_expr_right(void* p) { return ((ast_Expr_t*)p)->right; }
void* ql_ast_expr_args(void* p) { return ((ast_Expr_t*)p)->args; }

/* ========== CHECKED ARITH ========== */
#if defined(__GNUC__) || defined(__clang__)
ql_result_i32_i32 ql_add_checked_i32(int32_t a, int32_t b) {
    int32_t r;
    if (__builtin_add_overflow(a, b, &r)) {
        ql_result_i32_i32 x = { 0, { .err = 0 } };
        return x;
    }
        ql_result_i32_i32 x = { 1, { .val = r } };
    return x;
}
ql_result_i32_i32 ql_sub_checked_i32(int32_t a, int32_t b) {
    int32_t r;
    if (__builtin_sub_overflow(a, b, &r)) {
        ql_result_i32_i32 x = { 0, { .err = 0 } };
        return x;
    }
        ql_result_i32_i32 x = { 1, { .val = r } };
    return x;
}
ql_result_i32_i32 ql_mul_checked_i32(int32_t a, int32_t b) {
    int32_t r;
    if (__builtin_mul_overflow(a, b, &r)) {
        ql_result_i32_i32 x = { 0, { .err = 0 } };
        return x;
    }
        ql_result_i32_i32 x = { 1, { .val = r } };
    return x;
}
#else
ql_result_i32_i32 ql_add_checked_i32(int32_t a, int32_t b) {
    int64_t r = (int64_t)a + (int64_t)b;
    if (r < INT32_MIN || r > INT32_MAX) {
        ql_result_i32_i32 x = { 0, { .err = 0 } };
        return x;
    }
        ql_result_i32_i32 x = { 1, { .val = (int32_t)r } };
    return x;
}
ql_result_i32_i32 ql_sub_checked_i32(int32_t a, int32_t b) {
    int64_t r = (int64_t)a - (int64_t)b;
    if (r < INT32_MIN || r > INT32_MAX) {
        ql_result_i32_i32 x = { 0, { .err = 0 } };
        return x;
    }
        ql_result_i32_i32 x = { 1, { .val = (int32_t)r } };
    return x;
}
ql_result_i32_i32 ql_mul_checked_i32(int32_t a, int32_t b) {
    int64_t r = (int64_t)a * (int64_t)b;
    if (r < INT32_MIN || r > INT32_MAX) {
        ql_result_i32_i32 x = { 0, { .err = 0 } };
        return x;
    }
        ql_result_i32_i32 x = { 1, { .val = (int32_t)r } };
    return x;
}
#endif

/* ========== TIME ========== */
int64_t ql_time_now(void) { return (int64_t)time(0); }
