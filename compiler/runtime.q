// C runtime for ql_* functions - emitted by codegen when building self-contained output
// Bring "compiler.runtime" from codegen

bring "str";

realm runtime {
    craft emit() -> str {
        make str_rt: str = "static char* ql_str_trim(const char* s) {
    if (!s) return (char*)\"\";
    while (*s == ' ' || *s == '\\t' || *s == '\\n' || *s == '\\r') s++;
    const char* end = s;
    while (*end) end++;
    while (end > s && (end[-1] == ' ' || end[-1] == '\\t' || end[-1] == '\\n' || end[-1] == '\\r')) end--;
    size_t n = end - s;
    char* r = (char*)malloc(n + 1);
    memcpy(r, s, n);
    r[n] = 0;
    return r;
}
static char* ql_str_concat(const char* a, const char* b) {
    if (!a) a = \"\";
    if (!b) b = \"\";
    size_t la = strlen(a), lb = strlen(b);
    char* r = (char*)malloc(la + lb + 1);
    memcpy(r, a, la + 1);
    strcat(r, b);
    return r;
}
";
        make vec_rt: str = "typedef struct { void** data; size_t len; size_t cap; } ql_vec_ptr_t;
typedef struct { char* data; size_t len; size_t cap; } ql_vec_u8_t;
static void* ql_vec_ptr_new(void) {
    ql_vec_ptr_t* v = (ql_vec_ptr_t*)malloc(sizeof(ql_vec_ptr_t));
    v->data = 0; v->len = 0; v->cap = 0;
    return v;
}
static void ql_vec_ptr_push(void* vp, void* ptr) {
    ql_vec_ptr_t* v = (ql_vec_ptr_t*)vp;
    if (v->len >= v->cap) {
        size_t ncap = v->cap ? v->cap * 2 : 16;
        v->data = (void**)realloc(v->data, ncap * sizeof(void*));
        v->cap = ncap;
    }
    v->data[v->len++] = ptr;
}
static void* ql_vec_ptr_get(void* vp, size_t i) {
    ql_vec_ptr_t* v = (ql_vec_ptr_t*)vp;
    return (i < v->len) ? v->data[i] : 0;
}
static size_t ql_vec_ptr_len(void* vp) { return ((ql_vec_ptr_t*)vp)->len; }
static void ql_vec_ptr_clear(void* vp) { ((ql_vec_ptr_t*)vp)->len = 0; }
static void ql_vec_ptr_free(void* vp) {
    ql_vec_ptr_t* v = (ql_vec_ptr_t*)vp;
    free(v->data);
    free(v);
}
static void* ql_vec_u8_new(void) {
    ql_vec_u8_t* v = (ql_vec_u8_t*)malloc(sizeof(ql_vec_u8_t));
    v->data = 0; v->len = 0; v->cap = 0;
    return v;
}
static void ql_vec_u8_push(void* vp, unsigned char b) {
    ql_vec_u8_t* v = (ql_vec_u8_t*)vp;
    if (v->len >= v->cap) {
        size_t ncap = v->cap ? v->cap * 2 : 64;
        v->data = (char*)realloc(v->data, ncap);
        v->cap = ncap;
    }
    v->data[v->len++] = (char)b;
}
static void ql_vec_u8_append(void* vp, const char* s) {
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
static size_t ql_vec_u8_len(void* vp) { return ((ql_vec_u8_t*)vp)->len; }
static char* ql_vec_u8_to_str(void* vp) {
    ql_vec_u8_t* v = (ql_vec_u8_t*)vp;
    char* r = (char*)malloc(v->len + 1);
    memcpy(r, v->data, v->len);
    r[v->len] = 0;
    return r;
}
static void ql_vec_u8_clear(void* vp) { ((ql_vec_u8_t*)vp)->len = 0; }
static void ql_vec_u8_free(void* vp) {
    ql_vec_u8_t* v = (ql_vec_u8_t*)vp;
    free(v->data);
    free(v);
}
";
        make map_rt: str = "typedef struct { char* key; void* value; } ql_map_pair_t;
static void* ql_map_str_ptr_new(void) { return ql_vec_ptr_new(); }
static void ql_map_str_ptr_put(void* mp, const char* key, void* value) {
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
static void* ql_map_str_ptr_get(void* mp, const char* key) {
    ql_vec_ptr_t* v = (ql_vec_ptr_t*)mp;
    size_t i;
    for (i = 0; i < v->len; i++) {
        ql_map_pair_t* p = (ql_map_pair_t*)v->data[i];
        if (p && p->key && key && strcmp(p->key, key) == 0)
            return p->value;
    }
    return 0;
}
static int ql_map_str_ptr_has(void* mp, const char* key) {
    return ql_map_str_ptr_get(mp, key) != 0;
}
static size_t ql_map_str_ptr_len(void* mp) {
    return ql_vec_ptr_len(mp);
}
static void ql_map_str_ptr_free(void* mp) {
    ql_vec_ptr_t* v = (ql_vec_ptr_t*)mp;
    size_t i;
    for (i = 0; i < v->len; i++) {
        ql_map_pair_t* p = (ql_map_pair_t*)v->data[i];
        if (p) { free(p->key); free(p); }
    }
    ql_vec_ptr_free(mp);
}
";
        make fmt_rt: str = "static int ql_fmt_sprintf_s(char* buf, size_t size, const char* fmt, const char* s) {
    return snprintf(buf, size, fmt, s ? s : \"\");
}
static int ql_fmt_sprintf_ii(char* buf, size_t size, const char* fmt, long a, long b) {
    return snprintf(buf, size, fmt, a, b);
}
static int ql_fmt_sprintf_si(char* buf, size_t size, const char* fmt, const char* s, long a) {
    return snprintf(buf, size, fmt, s ? s : \"\", a);
}
static int ql_fmt_sprintf_ss(char* buf, size_t size, const char* fmt, const char* a, const char* b) {
    return snprintf(buf, size, fmt, a ? a : \"\", b ? b : \"\");
}
static char* ql_fmt_alloc_i(const char* fmt, long a) {
    char buf[64];
    int n = snprintf(buf, sizeof(buf), fmt, a);
    char* r = (char*)malloc((size_t)n + 1);
    memcpy(r, buf, (size_t)n + 1);
    return r;
}
static char* ql_fmt_alloc_s(const char* fmt, const char* s) {
    size_t n = strlen(s ? s : \"\") + 64;
    char* r = (char*)malloc(n);
    snprintf(r, n, fmt, s ? s : \"\");
    return r;
}
static char* ql_fmt_alloc_si(const char* fmt, const char* s, long a) {
    size_t n = strlen(s ? s : \"\") + 64;
    char* r = (char*)malloc(n);
    snprintf(r, n, fmt, s ? s : \"\"\", a);
    return r;
}
";
        make lex_rt: str = "typedef struct { int ty; size_t line; size_t col; char* str_val; long int_val; } ql_token_t;
static void* ql_token_create(int ty, size_t line, size_t col, const char* str_val, long int_val) {
    ql_token_t* t = (ql_token_t*)malloc(sizeof(ql_token_t));
    t->ty = ty; t->line = line; t->col = col;
    t->str_val = str_val ? strdup(str_val) : 0;
    t->int_val = int_val;
    return t;
}
static int ql_token_ty(void* t) { return ((ql_token_t*)t)->ty; }
static size_t ql_token_line(void* t) { return ((ql_token_t*)t)->line; }
static size_t ql_token_col(void* t) { return ((ql_token_t*)t)->col; }
static char* ql_token_str(void* t) { return ((ql_token_t*)t)->str_val; }
static long ql_token_int(void* t) { return ((ql_token_t*)t)->int_val; }
static void ql_token_free(void* t) {
    ql_token_t* tok = (ql_token_t*)t;
    free(tok->str_val);
    free(tok);
}
static int ql_str_at(const char* s, size_t i) {
    if (!s || i >= strlen(s)) return -1;
    return (unsigned char)s[i];
}
static char* ql_str_sub(const char* s, size_t start, size_t end) {
    if (!s || start >= end || end > strlen(s)) return strdup(\"\");
    size_t n = end - start;
    char* r = (char*)malloc(n + 1);
    memcpy(r, s + start, n);
    r[n] = 0;
    return r;
}
static void* ql_usize_to_ptr(size_t u) { return (void*)(uintptr_t)u; }
static size_t ql_ptr_to_usize(void* p) { return (size_t)(uintptr_t)p; }
";
        make ast_rt: str = "typedef struct { int tag; long int_val; char* str_val; void* left; void* right; void* args; } ast_Expr_t;
static void* ql_ast_expr_alloc(void) {
    return malloc(sizeof(ast_Expr_t));
}
static void ql_ast_expr_set_tag(void* p, int tag) { ((ast_Expr_t*)p)->tag = tag; }
static void ql_ast_expr_set_int(void* p, long val) { ((ast_Expr_t*)p)->int_val = val; }
static void ql_ast_expr_set_str(void* p, char* s) { ((ast_Expr_t*)p)->str_val = s; }
static void ql_ast_expr_set_left(void* p, void* left) { ((ast_Expr_t*)p)->left = left; }
static void ql_ast_expr_set_right(void* p, void* right) { ((ast_Expr_t*)p)->right = right; }
static void ql_ast_expr_set_args(void* p, void* args) { ((ast_Expr_t*)p)->args = args; }
static int ql_ast_expr_tag(void* p) { return ((ast_Expr_t*)p)->tag; }
static long ql_ast_expr_int(void* p) { return ((ast_Expr_t*)p)->int_val; }
static char* ql_ast_expr_str(void* p) { return ((ast_Expr_t*)p)->str_val; }
static void* ql_ast_expr_left(void* p) { return ((ast_Expr_t*)p)->left; }
static void* ql_ast_expr_right(void* p) { return ((ast_Expr_t*)p)->right; }
static void* ql_ast_expr_args(void* p) { return ((ast_Expr_t*)p)->args; }
";
        make r1: str = str.concat(str_rt, vec_rt);
        make r2: str = str.concat(r1, map_rt);
        make r3: str = str.concat(r2, fmt_rt);
        make r4: str = str.concat(r3, lex_rt);
        send str.concat(r4, ast_rt);
    }
}
