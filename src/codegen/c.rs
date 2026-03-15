//! C code generator - transpiles to C, compiled with system compiler

use crate::ast::{EnumVariant, *};
use crate::error::Result;
use crate::semantic::AnnotatedProgram;
use std::collections::HashMap;

#[derive(Clone)]
struct Ctx {
    vars: HashMap<String, String>,
    var_types: HashMap<String, Type>,
    program: Option<std::sync::Arc<crate::ast::Program>>,
    symbol_table: Option<std::sync::Arc<crate::semantic::SymbolTable>>,
    tuple_typedefs: std::collections::HashMap<String, String>,
}

impl Default for Ctx {
    fn default() -> Self {
        Self { vars: HashMap::new(), var_types: HashMap::new(), program: None, symbol_table: None, tuple_typedefs: std::collections::HashMap::new() }
    }
}

fn lookup_c_name(symbol_table: &crate::semantic::SymbolTable, name: &str) -> String {
    for scope in symbol_table.scopes.iter().rev() {
        if let Some(sig) = scope.funcs.get(name) {
            return sig.c_name.as_ref().cloned().unwrap_or_else(|| name.to_string());
        }
    }
    name.to_string()
}

fn type_to_c(ty: &Type) -> String {
    match ty {
        Type::Int => "long".to_string(),
        Type::Float => "double".to_string(),
        Type::Bool => "int".to_string(),
        Type::Str => "char*".to_string(),
        Type::Void => "void".to_string(),
        Type::Array(_) => "long*".to_string(),
        Type::ArraySized(inner, _) => type_to_c(inner).trim_end_matches('*').to_string(),
        Type::Named(name) => match name.as_str() {
            "Point" => "Point*".to_string(),
            _ => "void*".to_string(),
        },
        Type::U8 => "uint8_t".to_string(),
        Type::U16 => "uint16_t".to_string(),
        Type::U32 => "uint32_t".to_string(),
        Type::U64 => "uint64_t".to_string(),
        Type::I8 => "int8_t".to_string(),
        Type::I16 => "int16_t".to_string(),
        Type::I32 => "int32_t".to_string(),
        Type::I64 => "int64_t".to_string(),
        Type::Usize => "size_t".to_string(),
        Type::F32 => "float".to_string(),
        Type::F64 => "double".to_string(),
        Type::Ptr(inner) => format!("{}*", type_to_c(inner).trim_end_matches('*')),
        Type::Tuple(inner) => {
            let name = tuple_typedef_name(inner);
            format!("struct {{ {}; }}", inner.iter().enumerate()
                .map(|(i, t)| format!("{} _{}", type_to_c(t), i))
                .collect::<Vec<_>>()
                .join("; "))
        }
    }
}

fn resolve_type(ty: &Type, ctx: &Ctx) -> Type {
    if let Type::Named(name) = ty {
        if let Some(st) = ctx.symbol_table.as_ref() {
            for scope in st.scopes.iter().rev() {
                if let Some(aliased) = scope.aliases.get(name) {
                    return resolve_type(aliased, ctx);
                }
            }
        }
    }
    ty.clone()
}

fn type_to_c_with_typedef(ty: &Type, ctx: &Ctx) -> String {
    let ty = resolve_type(ty, ctx);
    if let Type::Tuple(inner) = &ty {
        let name = tuple_typedef_name(inner);
        if ctx.tuple_typedefs.contains_key(&name) {
            return name;
        }
    }
    type_to_c(&ty)
}

fn decl_to_c(ty: &Type, name: &str) -> String {
    match ty {
        Type::ArraySized(inner, n) => format!("{} {}[{}]", type_to_c(inner).trim_end_matches('*'), name, n),
        _ => format!("{} {}", type_to_c(ty), name),
    }
}

fn decl_to_c_with_ctx(ty: &Type, name: &str, ctx: &Ctx) -> String {
    match ty {
        Type::ArraySized(inner, n) => format!("{} {}[{}]", type_to_c(inner).trim_end_matches('*'), name, n),
        _ => format!("{} {}", type_to_c_with_typedef(ty, ctx), name),
    }
}

fn type_to_printf(ty: &Type) -> &'static str {
    match ty {
        Type::Int | Type::I32 | Type::I64 => "%ld",
        Type::U8 | Type::U16 | Type::U32 | Type::U64 | Type::Usize => "%lu",
        Type::Float | Type::F32 | Type::F64 => "%f",
        Type::Bool => "%d",
        Type::Str => "%s",
        Type::Void => "%s",
        Type::Array(_) | Type::ArraySized(_, _) | Type::Named(_) => "%p",
        Type::I8 | Type::I16 => "%d",
        Type::Ptr(_) => "%p",
        Type::Tuple(_) => "%p",
    }
}

fn expr_type(expr: &Expr, ctx: &Ctx) -> Option<Type> {
    match expr {
        Expr::Literal(l) => Some(match l {
            Literal::Int(_) => Type::Int,
            Literal::Float(_) => Type::Float,
            Literal::Bool(_) => Type::Bool,
            Literal::Str(_) => Type::Str,
        }),
        Expr::Ident(name) => ctx.var_types.get(name).cloned(),
        Expr::AddrOf(operand) => expr_type(operand, ctx).map(|t| Type::Ptr(Box::new(t))),
        Expr::Deref(operand) => match expr_type(operand, ctx) {
            Some(Type::Ptr(t)) => Some(*t),
            _ => None,
        },
        Expr::Field { base, .. } => expr_type(base, ctx),
        Expr::Call { callee, .. } => {
            if let Some(st) = ctx.symbol_table.as_ref() {
                if let Expr::Ident(name) = callee.as_ref() {
                    for scope in st.scopes.iter().rev() {
                        if let Some(sig) = scope.funcs.get(name) {
                            return sig.return_type.clone();
                        }
                    }
                }
                if let Expr::Field { base, field } = callee.as_ref() {
                    if let Expr::Ident(mod_name) = base.as_ref() {
                        if let Some(mod_funcs) = st.mod_functions.get(mod_name) {
                            if let Some(sig) = mod_funcs.get(field) {
                                return sig.return_type.clone();
                            }
                        }
                    }
                }
            }
            None
        }
        Expr::Tuple(elems) => {
            let mut tys = Vec::new();
            for e in elems {
                if let Some(t) = expr_type(e, ctx) {
                    tys.push(t);
                } else {
                    return None;
                }
            }
            Some(Type::Tuple(tys))
        }
        Expr::Cast { target_ty, .. } => Some(target_ty.clone()),
        Expr::Binary { op, left, right } => {
            let lt = expr_type(left, ctx)?;
            let rt = expr_type(right, ctx)?;
            match op {
                BinOp::Eq | BinOp::Ne | BinOp::Lt | BinOp::Le | BinOp::Gt | BinOp::Ge
                | BinOp::And | BinOp::Or => Some(Type::Bool),
                _ => Some(lt),
            }
        }
        _ => None,
    }
}

fn tuple_typedef_name(inner: &[Type]) -> String {
    let parts: Vec<String> = inner.iter().map(|t| {
        match t {
            Type::Int => "long".to_string(),
            Type::I32 => "i32".to_string(),
            Type::I64 => "i64".to_string(),
            Type::Usize => "usz".to_string(),
            Type::Str => "str".to_string(),
            Type::Bool => "bool".to_string(),
            Type::Float | Type::F32 | Type::F64 => "flt".to_string(),
            _ => "p".to_string(),
        }
    }).collect();
    format!("tuple_{}", parts.join("_"))
}

pub fn generate(program: &AnnotatedProgram) -> Result<String> {
    let mut out = String::new();
    out.push_str("#include <stdlib.h>\n");
    out.push_str("#include <stdint.h>\n");
    out.push_str("#include <stdio.h>\n");
    out.push_str("#include <string.h>\n");
    out.push_str("#include <math.h>\n");
    out.push_str("#ifdef _WIN32\n#include <direct.h>\n#define getcwd _getcwd\n#else\n#include <unistd.h>\n#endif\n");

    // Emit str runtime if str module is used
    if program_uses_module(&program.program, "str") {
        out.push_str(STR_RUNTIME);
    }
    // Emit vec runtime if vec module is used
    if program_uses_module(&program.program, "vec") {
        out.push_str(VEC_RUNTIME);
    }
    // Emit lex runtime if lexer module is used (for compiler bootstrap)
    if program_uses_module(&program.program, "lexer") {
        if !program_uses_module(&program.program, "vec") {
            out.push_str(VEC_RUNTIME);
        }
        out.push_str(LEX_RUNTIME);
    }
    // Emit fmt runtime if fmt module is used
    if program_uses_module(&program.program, "fmt") {
        out.push_str(FMT_RUNTIME);
    }
    // Emit map runtime if map module is used (depends on vec)
    if program_uses_module(&program.program, "map") {
        if !program_uses_module(&program.program, "vec") {
            out.push_str(VEC_RUNTIME);
        }
        out.push_str(MAP_RUNTIME);
    }
    // Emit AST runtime if ast module is used (for compiler bootstrap)
    if program_uses_module(&program.program, "ast") {
        out.push_str(AST_RUNTIME);
    }

    let mut tuple_typedefs: std::collections::HashSet<String> = std::collections::HashSet::new();
    for item in &program.program.items {
        if let TopLevelItem::Fn(f) = item {
            if let Some(Type::Tuple(inner)) = &f.return_type {
                let name = tuple_typedef_name(inner);
                if !tuple_typedefs.contains(&name) {
                    tuple_typedefs.insert(name.clone());
                    let parts: Vec<String> = inner.iter().enumerate()
                        .map(|(i, t)| format!("{} _{}", type_to_c(t), i))
                        .collect();
                    out.push_str(&format!("typedef struct {{ {}; }} {};\n", parts.join("; "), name));
                }
            }
        }
    }

    let mut ctx = Ctx::default();
    ctx.program = Some(std::sync::Arc::new(program.program.clone()));
    ctx.symbol_table = Some(std::sync::Arc::new(program.symbol_table.clone()));
    for item in &program.program.items {
        if let TopLevelItem::Fn(f) = item {
            if let Some(Type::Tuple(inner)) = &f.return_type {
                let name = tuple_typedef_name(inner);
                let parts: Vec<String> = inner.iter().enumerate()
                    .map(|(i, t)| format!("{} _{}", type_to_c(t), i))
                    .collect();
                ctx.tuple_typedefs.insert(name.clone(), format!("struct {{ {}; }}", parts.join("; ")));
            }
        }
    }
    // First pass: emit modules (so their functions are defined before main)
    for item in &program.program.items {
        if matches!(item, TopLevelItem::Mod(_)) {
            emit_top_level(&mut out, item, &mut ctx)?;
        }
    }
    // Second pass: emit top-level items except modules
    for item in &program.program.items {
        if !matches!(item, TopLevelItem::Mod(_)) {
            emit_top_level(&mut out, item, &mut ctx)?;
        }
    }
    Ok(out)
}

const STR_RUNTIME: &str = r#"
static char* ql_str_trim(const char* s) {
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
static char* ql_str_concat(const char* a, const char* b) {
    if (!a) a = "";
    if (!b) b = "";
    size_t la = strlen(a), lb = strlen(b);
    char* r = (char*)malloc(la + lb + 1);
    memcpy(r, a, la + 1);
    strcat(r, b);
    return r;
}
"#;

const VEC_RUNTIME: &str = r#"
typedef struct { void** data; size_t len; size_t cap; } ql_vec_ptr_t;
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
"#;

const MAP_RUNTIME: &str = r#"
typedef struct { char* key; void* value; } ql_map_pair_t;
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
"#;

const FMT_RUNTIME: &str = r#"
static int ql_fmt_sprintf_s(char* buf, size_t size, const char* fmt, const char* s) {
    return snprintf(buf, size, fmt, s ? s : "");
}
static int ql_fmt_sprintf_ii(char* buf, size_t size, const char* fmt, long a, long b) {
    return snprintf(buf, size, fmt, a, b);
}
static int ql_fmt_sprintf_si(char* buf, size_t size, const char* fmt, const char* s, long a) {
    return snprintf(buf, size, fmt, s ? s : "", a);
}
static int ql_fmt_sprintf_ss(char* buf, size_t size, const char* fmt, const char* a, const char* b) {
    return snprintf(buf, size, fmt, a ? a : "", b ? b : "");
}
static char* ql_fmt_alloc_i(const char* fmt, long a) {
    char buf[64];
    int n = snprintf(buf, sizeof(buf), fmt, a);
    char* r = (char*)malloc((size_t)n + 1);
    memcpy(r, buf, (size_t)n + 1);
    return r;
}
static char* ql_fmt_alloc_s(const char* fmt, const char* s) {
    size_t n = strlen(s ? s : "") + 64;
    char* r = (char*)malloc(n);
    snprintf(r, n, fmt, s ? s : "");
    return r;
}
static char* ql_fmt_alloc_si(const char* fmt, const char* s, long a) {
    size_t n = strlen(s ? s : "") + 64;
    char* r = (char*)malloc(n);
    snprintf(r, n, fmt, s ? s : "", a);
    return r;
}
"#;

const LEX_RUNTIME: &str = r#"
typedef struct { int ty; size_t line; size_t col; char* str_val; long int_val; } ql_token_t;
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
    if (!s || start >= end || end > strlen(s)) return strdup("");
    size_t n = end - start;
    char* r = (char*)malloc(n + 1);
    memcpy(r, s + start, n);
    r[n] = 0;
    return r;
}
static void* ql_usize_to_ptr(size_t u) { return (void*)(uintptr_t)u; }
static size_t ql_ptr_to_usize(void* p) { return (size_t)(uintptr_t)p; }
"#;

const AST_RUNTIME: &str = r#"
typedef struct { int tag; long int_val; char* str_val; void* left; void* right; void* args; } ast_Expr_t;
static void* ql_ast_expr_alloc(void) {
    return malloc(sizeof(ast_Expr_t));
}
static void ql_ast_expr_set_tag(void* p, int tag) { ((ast_Expr_t*)p)->tag = tag; }
static void ql_ast_expr_set_int(void* p, long val) { ((ast_Expr_t*)p)->int_val = val; }
static void ql_ast_expr_set_str(void* p, char* s) { ((ast_Expr_t*)p)->str_val = s; }
static void ql_ast_expr_set_left(void* p, void* left) { ((ast_Expr_t*)p)->left = left; }
static void ql_ast_expr_set_right(void* p, void* right) { ((ast_Expr_t*)p)->right = right; }
static int ql_ast_expr_tag(void* p) { return ((ast_Expr_t*)p)->tag; }
static long ql_ast_expr_int(void* p) { return ((ast_Expr_t*)p)->int_val; }
static char* ql_ast_expr_str(void* p) { return ((ast_Expr_t*)p)->str_val; }
static void* ql_ast_expr_left(void* p) { return ((ast_Expr_t*)p)->left; }
static void* ql_ast_expr_right(void* p) { return ((ast_Expr_t*)p)->right; }
"#;

fn program_uses_module(program: &crate::ast::Program, name: &str) -> bool {
    for item in &program.items {
        match item {
            crate::ast::TopLevelItem::Import(i) if i.path.first().map(|s| s.as_str()) == Some(name) => return true,
            crate::ast::TopLevelItem::Mod(m) if m.name == name => return true,
            _ => {}
        }
    }
    false
}

fn find_method_struct(program: &crate::ast::Program, method_name: &str) -> Option<String> {
    for item in &program.items {
        if let TopLevelItem::Impl(impl_def) = item {
            if impl_def.methods.iter().any(|m| m.name == method_name) {
                return Some(impl_def.struct_name.clone());
            }
        }
    }
    None
}

fn find_module_function(program: &crate::ast::Program, mod_name: &str, fn_name: &str) -> bool {
    for item in &program.items {
        if let TopLevelItem::Mod(m) = item {
            if m.name == mod_name {
                return m.items.iter().any(|sub| {
                    matches!(sub, TopLevelItem::Fn(f) if f.name == fn_name)
                });
            }
        }
    }
    false
}

fn emit_top_level(out: &mut String, item: &TopLevelItem, ctx: &mut Ctx) -> Result<()> {
    emit_top_level_with_prefix(out, item, ctx, None)
}

fn emit_top_level_with_prefix(out: &mut String, item: &TopLevelItem, ctx: &mut Ctx, mod_prefix: Option<&str>) -> Result<()> {
    match item {
        TopLevelItem::Const(c) => {
            let c_name = mod_prefix.map(|p| format!("{}_{}", p, c.name)).unwrap_or_else(|| c.name.clone());
            out.push_str(&format!("static const {} = ", decl_to_c(&c.ty, &c_name)));
            emit_expr(out, &c.init, ctx)?;
            out.push_str(";\n\n");
        }
        TopLevelItem::Static(s) => {
            let s_name = mod_prefix.map(|p| format!("{}_{}", p, s.name)).unwrap_or_else(|| s.name.clone());
            out.push_str(&format!("static {} ", decl_to_c(&s.ty, &s_name)));
            if let Some(init) = &s.init {
                out.push_str(" = ");
                emit_expr(out, init, ctx)?;
            }
            out.push_str(";\n\n");
        }
        TopLevelItem::Fn(f) => {
            let name = mod_prefix.map(|p| format!("{}_{}", p, f.name)).unwrap_or_else(|| f.name.clone());
            emit_fn_named(out, f, ctx, &name)?;
        }
        TopLevelItem::Struct(s) => emit_struct(out, s)?,
        TopLevelItem::Class(c) => emit_class(out, c, ctx)?,
        TopLevelItem::Enum(e) => emit_enum(out, e)?,
        TopLevelItem::Union(u) => emit_union(out, u)?,
        TopLevelItem::Mod(m) => {
            let prefix = Some(m.name.as_str());
            for sub in &m.items {
                if let TopLevelItem::Fn(f) = sub {
                    let name = prefix.map(|p| format!("{}_{}", p, f.name)).unwrap_or_else(|| f.name.clone());
                    let ret = f.return_type.as_ref().map(type_to_c).unwrap_or_else(|| "void".to_string());
                    out.push_str(&format!("static {} {}(", ret, name));
                    for (i, p) in f.params.iter().enumerate() {
                        if i > 0 { out.push_str(", "); }
                        out.push_str(&format!("{} {}", type_to_c(&p.ty), p.name));
                    }
                    out.push_str(");\n");
                }
            }
            out.push_str("\n");
            for sub in &m.items {
                emit_top_level_with_prefix(out, sub, ctx, Some(&m.name))?;
            }
        }
        TopLevelItem::Import(_) => {}
        TopLevelItem::Alias(_) => {}
        TopLevelItem::Extern(e) => {
            const STDLIB_FUNCS: &[&str] = &["fopen", "fclose", "fread", "fwrite", "malloc", "free", "realloc", "fseek", "ftell", "system", "getenv", "getcwd", "abs", "fabs", "sqrt", "fmin", "fmax", "ql_str_trim", "ql_str_concat", "ql_vec_ptr_new", "ql_vec_ptr_push", "ql_vec_ptr_get", "ql_vec_ptr_len", "ql_vec_ptr_clear", "ql_vec_ptr_free", "ql_vec_u8_new", "ql_vec_u8_push", "ql_vec_u8_append", "ql_vec_u8_len", "ql_vec_u8_to_str", "ql_vec_u8_clear", "ql_vec_u8_free", "ql_map_str_ptr_new", "ql_map_str_ptr_put", "ql_map_str_ptr_get", "ql_map_str_ptr_has", "ql_map_str_ptr_len", "ql_map_str_ptr_free", "snprintf", "ql_fmt_sprintf_s", "ql_fmt_sprintf_ii", "ql_fmt_sprintf_si", "ql_fmt_sprintf_ss", "ql_fmt_alloc_i", "ql_fmt_alloc_s", "ql_fmt_alloc_si", "ql_token_create", "ql_token_ty", "ql_token_line", "ql_token_col", "ql_token_str", "ql_token_int", "ql_token_free", "ql_str_at", "ql_str_sub", "ql_usize_to_ptr", "ql_ptr_to_usize",
            "ql_ast_expr_alloc", "ql_ast_expr_set_tag", "ql_ast_expr_set_int", "ql_ast_expr_set_str", "ql_ast_expr_set_left", "ql_ast_expr_set_right",
            "ql_ast_expr_tag", "ql_ast_expr_int", "ql_ast_expr_str", "ql_ast_expr_left", "ql_ast_expr_right"];
            if STDLIB_FUNCS.contains(&e.name.as_str()) {
                return Ok(());
            }
            let ret = e.return_type.as_ref().map(type_to_c).unwrap_or_else(|| "void".to_string());
            out.push_str(&format!("extern {} {}(", ret, e.name));
            for (i, p) in e.params.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                out.push_str(&format!("{} {}", type_to_c(&p.ty), p.name));
            }
            out.push_str(");\n\n");
        }
        TopLevelItem::Impl(impl_def) => {
            for m in &impl_def.methods {
                let ret = m.return_type.as_ref().map(type_to_c).unwrap_or_else(|| "void".to_string());
                let struct_ty = impl_def.struct_name.clone();
                out.push_str(&format!("{} {}_{}(", ret, struct_ty, m.name));
                out.push_str(&format!("{}* self", struct_ty));
                for p in &m.params {
                    if p.name != "self" {
                        out.push_str(&format!(", {} {}", type_to_c(&p.ty), p.name));
                    }
                }
                out.push_str(") {\n");
                let mut fn_ctx = Ctx::default();
                fn_ctx.program = ctx.program.clone();
                fn_ctx.vars.insert("self".to_string(), "self".to_string());
                fn_ctx.var_types.insert("self".to_string(), Type::Ptr(Box::new(Type::Named(impl_def.struct_name.clone()))));
                for p in &m.params {
                    if p.name != "self" {
                        fn_ctx.vars.insert(p.name.clone(), p.name.clone());
                        fn_ctx.var_types.insert(p.name.clone(), p.ty.clone());
                    }
                }
                for stmt in &m.body {
                    emit_stmt(out, stmt, &mut fn_ctx)?;
                }
                out.push_str("}\n\n");
            }
        }
    }
    Ok(())
}

fn emit_enum(out: &mut String, e: &EnumDef) -> Result<()> {
    out.push_str(&format!("typedef enum {{\n"));
    for (i, v) in e.variants.iter().enumerate() {
        let vname = match v {
            EnumVariant::Unit(n) => n.clone(),
            EnumVariant::Tuple(n, _) => n.clone(),
        };
        out.push_str(&format!("    {} = {}", vname, i));
        if i < e.variants.len() - 1 {
            out.push_str(",");
        }
        out.push_str("\n");
    }
    out.push_str(&format!("}} {};\n\n", e.name));
    Ok(())
}

fn emit_union(out: &mut String, u: &UnionDef) -> Result<()> {
    out.push_str(&format!("typedef union {{\n"));
    for f in &u.fields {
        out.push_str(&format!("    {} {};\n", type_to_c(&f.ty), f.name));
    }
    out.push_str(&format!("}} {};\n\n", u.name));
    Ok(())
}

fn emit_struct(out: &mut String, s: &StructDef) -> Result<()> {
    out.push_str("typedef struct {\n");
    for f in &s.fields {
        out.push_str(&format!("    {} {};\n", type_to_c(&f.ty), f.name));
    }
    out.push_str("} ");
    out.push_str(&s.name);
    out.push_str(";\n\n");
    Ok(())
}

fn emit_class(out: &mut String, c: &ClassDef, _ctx: &mut Ctx) -> Result<()> {
    out.push_str("typedef struct {\n");
    for f in &c.fields {
        out.push_str(&format!("    {} {};\n", type_to_c(&f.ty), f.name));
    }
    out.push_str("} ");
    out.push_str(&c.name);
    out.push_str(";\n\n");

    if let Some(init) = &c.init {
        out.push_str(&format!("void {}_init({}* this", c.name, c.name));
        for p in &init.params {
            out.push_str(&format!(", {} {}", type_to_c(&p.ty), p.name));
        }
        out.push_str(") {\n");
        let mut init_ctx = Ctx::default();
        init_ctx.vars.insert("this".to_string(), "this".to_string());
        for p in &init.params {
            init_ctx.vars.insert(p.name.clone(), p.name.clone());
        }
        for stmt in &init.body {
            emit_stmt(out, stmt, &mut init_ctx)?;
        }
        out.push_str("}\n\n");
    }

    for m in &c.methods {
        let ret = m.return_type.as_ref().map(type_to_c).unwrap_or_else(|| "void".to_string());
        out.push_str(&format!("{} {}_{}({}* this", ret, c.name, m.name, c.name));
        for p in &m.params {
            out.push_str(&format!(", {} {}", type_to_c(&p.ty), p.name));
        }
        out.push_str(") {\n");
        let mut method_ctx = Ctx::default();
        method_ctx.vars.insert("this".to_string(), "this".to_string());
        for p in &m.params {
            method_ctx.vars.insert(p.name.clone(), p.name.clone());
        }
        for stmt in &m.body {
            emit_stmt(out, stmt, &mut method_ctx)?;
        }
        out.push_str("}\n\n");
    }
    Ok(())
}

fn emit_fn_named(out: &mut String, f: &FnDef, ctx: &mut Ctx, name: &str) -> Result<()> {
    let ret = f.return_type.as_ref().map(|t| type_to_c_with_typedef(t, ctx)).unwrap_or_else(|| "void".to_string());
    out.push_str(&format!("{} {}(", ret, name));
    for (i, p) in f.params.iter().enumerate() {
        if i > 0 {
            out.push_str(", ");
        }
        out.push_str(&format!("{} {}", type_to_c(&p.ty), p.name));
    }
    out.push_str(") {\n");
    let mut fn_ctx = Ctx::default();
    fn_ctx.program = ctx.program.clone();
    fn_ctx.symbol_table = ctx.symbol_table.clone();
    fn_ctx.tuple_typedefs = ctx.tuple_typedefs.clone();
    for p in &f.params {
        fn_ctx.vars.insert(p.name.clone(), p.name.clone());
        fn_ctx.var_types.insert(p.name.clone(), p.ty.clone());
    }
    for stmt in &f.body {
        emit_stmt(out, stmt, &mut fn_ctx)?;
    }
    out.push_str("}\n\n");
    Ok(())
}

fn emit_fn(out: &mut String, f: &FnDef, ctx: &mut Ctx) -> Result<()> {
    emit_fn_named(out, f, ctx, &f.name)
}

fn emit_stmt(out: &mut String, stmt: &Stmt, ctx: &mut Ctx) -> Result<()> {
    match stmt {
        Stmt::VarDecl { name, ty, init, mutable: _ } => {
            let decl_ty = ty.clone().unwrap_or(Type::Int);
            ctx.vars.insert(name.clone(), name.clone());
            ctx.var_types.insert(name.clone(), decl_ty.clone());
            let decl = decl_to_c(&decl_ty, name);
            out.push_str(&format!("    {} = ", decl));
            emit_expr(out, init, ctx)?;
            out.push_str(";\n");
        }
        Stmt::VarDeclTuple { names, init, mutable: _ } => {
            if let Some(Type::Tuple(elem_tys)) = expr_type(init, ctx) {
                if names.len() == elem_tys.len() {
                    let tmp_name = "_tuple_tmp";
                    let tuple_ty = Type::Tuple(elem_tys.clone());
                    let decl = decl_to_c_with_ctx(&tuple_ty, tmp_name, ctx);
                    out.push_str(&format!("    {} = ", decl));
                    emit_expr(out, init, ctx)?;
                    out.push_str(";\n");
                    for (i, (var_name, ty)) in names.iter().zip(elem_tys.iter()).enumerate() {
                        ctx.vars.insert(var_name.clone(), var_name.clone());
                        ctx.var_types.insert(var_name.clone(), ty.clone());
                        let field_decl = decl_to_c(ty, var_name);
                        out.push_str(&format!("    {} = {}._{};\n", field_decl, tmp_name, i));
                    }
                }
            }
        }
        Stmt::Assign { target, value } => {
            out.push_str("    ");
            emit_assign_target(out, target, ctx)?;
            out.push_str(" = ");
            emit_expr(out, value, ctx)?;
            out.push_str(";\n");
        }
        Stmt::If { cond, then_body, else_body } => {
            out.push_str("    if (");
            emit_expr(out, cond, ctx)?;
            out.push_str(") {\n");
            for s in then_body {
                emit_stmt(out, s, ctx)?;
            }
            out.push_str("    }");
            if let Some(else_b) = else_body {
                out.push_str(" else {\n");
                for s in else_b {
                    emit_stmt(out, s, ctx)?;
                }
                out.push_str("    }");
            }
            out.push_str("\n");
        }
        Stmt::For { init, cond, step, body } => {
            out.push_str("    for (");
            if let Some(i) = init {
                emit_for_init(out, i, ctx)?;
            }
            out.push_str("; ");
            if let Some(c) = cond {
                emit_expr(out, c, ctx)?;
            }
            out.push_str("; ");
            if let Some(s) = step {
                emit_for_step(out, s, ctx)?;
            }
            out.push_str(") {\n");
            for s in body {
                emit_stmt(out, s, ctx)?;
            }
            out.push_str("    }\n");
        }
        Stmt::While { cond, body } => {
            out.push_str("    while (");
            emit_expr(out, cond, ctx)?;
            out.push_str(") {\n");
            for s in body {
                emit_stmt(out, s, ctx)?;
            }
            out.push_str("    }\n");
        }
        Stmt::Foreach { var, iter, body } => {
            if let Expr::Range { start, end } = iter.as_ref() {
                out.push_str("    {\n");
                out.push_str(&format!("    int {};\n", var));
                out.push_str(&format!("    for ({} = ", var));
                emit_expr(out, start, ctx)?;
                out.push_str(&format!("; {} < ", var));
                emit_expr(out, end, ctx)?;
                out.push_str(&format!("; {}++) {{\n", var));
                ctx.vars.insert(var.clone(), var.clone());
                ctx.var_types.insert(var.clone(), Type::I32);
                for s in body {
                    emit_stmt(out, s, ctx)?;
                }
                out.push_str("    }\n");
                out.push_str("    }\n");
            } else {
                let idx = "_foreach_i";
                out.push_str("    {\n");
                out.push_str(&format!("    size_t {};\n", idx));
                out.push_str(&format!("    for ({} = 0; {} < sizeof(", idx, idx));
                emit_expr(out, iter, ctx)?;
                out.push_str(")/sizeof((");
                emit_expr(out, iter, ctx)?;
                out.push_str(")[0]); ");
                out.push_str(&format!("{}++) {{\n", idx));
                out.push_str("        long ");
                out.push_str(var);
                out.push_str(" = (");
                emit_expr(out, iter, ctx)?;
                out.push_str(")[");
                out.push_str(idx);
                out.push_str("];\n");
                ctx.vars.insert(var.clone(), var.clone());
                ctx.var_types.insert(var.clone(), Type::Int);
                for s in body {
                    emit_stmt(out, s, ctx)?;
                }
                out.push_str("    }\n");
                out.push_str("    }\n");
            }
        }
        Stmt::Break => out.push_str("    break;\n"),
        Stmt::Continue => out.push_str("    continue;\n"),
        Stmt::Hazard { body } => {
            for s in body {
                emit_stmt(out, s, ctx)?;
            }
        }
        Stmt::InlineAsm { instructions } => {
            for instr in instructions {
                let escaped = instr.replace('\\', "\\\\").replace('"', "\\\"");
                out.push_str(&format!(
                    "    #if defined(__GNUC__) || defined(__clang__)\n    __asm__ __volatile__(\"{}\");\n    #else\n    /* MSVC x64: inline asm not supported - use MinGW/Clang for machine blocks */\n    #endif\n",
                    escaped
                ));
            }
        }
        Stmt::ExprStmt(e) => {
            out.push_str("    ");
            emit_expr(out, e, ctx)?;
            out.push_str(";\n");
        }
        Stmt::Return(expr) => {
            out.push_str("    return");
            if let Some(e) = expr {
                out.push_str(" ");
                emit_expr(out, e, ctx)?;
            }
            out.push_str(";\n");
        }
        Stmt::TryCatch { try_body, .. } => {
            for s in try_body {
                emit_stmt(out, s, ctx)?;
            }
        }
        Stmt::Defer { body } => {
            for s in body {
                emit_stmt(out, s, ctx)?;
            }
        }
        Stmt::Choose { expr, arms } => {
            out.push_str("    {\n    int _choose_val = (int)(");
            emit_expr(out, expr, ctx)?;
            out.push_str(");\n");
            for (i, arm) in arms.iter().enumerate() {
                let prefix = if i == 0 { "    if " } else { "    else if " };
                let cond = match &arm.pattern {
                    ChoosePattern::UnitVariant(_v) => format!("_choose_val == {}", i),
                    ChoosePattern::TupleVariant(_, _) => format!("_choose_val == {}", i),
                    ChoosePattern::Ident(_) => "1".to_string(),
                };
                out.push_str(&format!("{}({}) {{\n", prefix, cond));
                for s in &arm.body {
                    emit_stmt(out, s, ctx)?;
                }
                out.push_str("    }\n");
            }
            out.push_str("    }\n");
        }
    }
    Ok(())
}

fn emit_for_init(out: &mut String, stmt: &Stmt, ctx: &mut Ctx) -> Result<()> {
    match stmt {
        Stmt::VarDecl { name, ty, init, mutable: _ } => {
            let decl_ty = ty.clone().unwrap_or(Type::Int);
            ctx.vars.insert(name.clone(), name.clone());
            ctx.var_types.insert(name.clone(), decl_ty.clone());
            let decl = decl_to_c(&decl_ty, name);
            out.push_str(&format!("{} = ", decl));
            emit_expr(out, init, ctx)?;
        }
        _ => {}
    }
    Ok(())
}

fn emit_for_step(out: &mut String, stmt: &Stmt, ctx: &Ctx) -> Result<()> {
    match stmt {
        Stmt::Assign { target, value } => {
            emit_assign_target(out, target, ctx)?;
            out.push_str(" = ");
            emit_expr(out, value, ctx)?;
        }
        _ => {}
    }
    Ok(())
}

fn emit_assign_target(out: &mut String, target: &AssignTarget, ctx: &Ctx) -> Result<()> {
    match target {
        AssignTarget::Ident(name) => out.push_str(name),
        AssignTarget::Field { base, field } => {
            emit_expr(out, base, ctx)?;
            out.push_str(&format!("->{}", field));
        }
        AssignTarget::Index { base, index } => {
            emit_expr(out, base, ctx)?;
            out.push_str("[");
            emit_expr(out, index, ctx)?;
            out.push_str("]");
        }
        AssignTarget::Deref(operand) => {
            out.push_str("*");
            emit_expr(out, operand, ctx)?;
        }
    }
    Ok(())
}

fn emit_expr(out: &mut String, expr: &Expr, ctx: &Ctx) -> Result<()> {
    match expr {
        Expr::Literal(Literal::Int(n)) => out.push_str(&n.to_string()),
        Expr::Literal(Literal::Float(_)) => out.push_str("0.0"),
        Expr::Literal(Literal::Bool(b)) => out.push_str(if *b { "1" } else { "0" }),
        Expr::Literal(Literal::Str(s)) => {
            out.push_str(&format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\"").replace('\n', "\\n").replace('\r', "\\r").replace('\t', "\\t")));
        }
        Expr::Ident(name) => out.push_str(name),
        Expr::Binary { op, left, right } => {
            let lt = expr_type(left, ctx);
            let rt = expr_type(right, ctx);
            if *op == BinOp::Add && lt == Some(Type::Str) && rt == Some(Type::Str) {
                out.push_str("({ const char* _a = ");
                emit_expr(out, left, ctx)?;
                out.push_str("; const char* _b = ");
                emit_expr(out, right, ctx)?;
                out.push_str("; size_t _la = strlen(_a); size_t _lb = strlen(_b); char* _r = (char*)malloc(_la + _lb + 1); memcpy(_r, _a, _la + 1); strcat(_r, _b); _r; })");
            } else {
            out.push_str("(");
            emit_expr(out, left, ctx)?;
            out.push_str(" ");
            out.push_str(match op {
                BinOp::Add => "+",
                BinOp::Sub => "-",
                BinOp::Mul => "*",
                BinOp::Div => "/",
                BinOp::Mod => "%",
                BinOp::Eq => "==",
                BinOp::Ne => "!=",
                BinOp::Lt => "<",
                BinOp::Le => "<=",
                BinOp::Gt => ">",
                BinOp::Ge => ">=",
                BinOp::And => "&&",
                BinOp::Or => "||",
            });
            out.push_str(" ");
            emit_expr(out, right, ctx)?;
            out.push_str(")");
            }
        }
        Expr::Unary { op, operand } => {
            out.push_str(match op {
                UnOp::Neg => "(-",
                UnOp::Not => "(!",
            });
            emit_expr(out, operand, ctx)?;
            out.push_str(")");
        }
        Expr::Call { callee, args } => {
            if let Expr::Field { base, field } = callee.as_ref() {
                if let Some(ref prog) = ctx.program {
                    if let Some(struct_name) = find_method_struct(prog, field) {
                        out.push_str(&format!("{}_{}(", struct_name, field));
                        let base_ty = expr_type(base, ctx);
                        let need_addr = matches!(base_ty, Some(Type::Named(_)));
                        if need_addr {
                            out.push_str("&(");
                        }
                        emit_expr(out, base, ctx)?;
                        if need_addr {
                            out.push_str(")");
                        }
                        for arg in args {
                            out.push_str(", ");
                            emit_expr(out, arg, ctx)?;
                        }
                        out.push_str(")");
                        return Ok(());
                    }
                    if let Expr::Ident(mod_name) = base.as_ref() {
                        if find_module_function(prog, mod_name, field) {
                            out.push_str(&format!("{}_{}(", mod_name, field));
                            for (i, arg) in args.iter().enumerate() {
                                if i > 0 {
                                    out.push_str(", ");
                                }
                                emit_expr(out, arg, ctx)?;
                            }
                            out.push_str(")");
                            return Ok(());
                        }
                    }
                }
            }
            let callee_name = callee.as_ref();
            let is_null_at = matches!(callee_name, Expr::Ident(n) if n == "__ql_null_at");
            let is_print = matches!(callee_name, Expr::Ident(n) if n == "print");
            let is_writeln = matches!(callee_name, Expr::Ident(n) if n == "writeln");
            let is_write = matches!(callee_name, Expr::Ident(n) if n == "write");
            let is_read = matches!(callee_name, Expr::Ident(n) if n == "read");
            let is_len = matches!(callee_name, Expr::Ident(n) if n == "len");
            let is_strlen = matches!(callee_name, Expr::Ident(n) if n == "strlen");
            let is_panic = matches!(callee_name, Expr::Ident(n) if n == "panic");
            let is_assert = matches!(callee_name, Expr::Ident(n) if n == "assert");
            if is_null_at {
                if let (Some(ptr), Some(offset)) = (args.get(0), args.get(1)) {
                    out.push_str("((void)(((char*)(");
                    emit_expr(out, ptr, ctx)?;
                    out.push_str("))[");
                    emit_expr(out, offset, ctx)?;
                    out.push_str("] = '\\0'))");
                }
            } else if is_print || is_writeln || is_write {
                if args.len() == 1 {
                    if let Expr::Interpolate(parts) = &args[0] {
                        let mut fmt_parts = Vec::new();
                        let mut expr_args = Vec::new();
                        for p in parts {
                            match p {
                                InterpolatePart::Str(s) => fmt_parts.push(s.replace('%', "%%").replace('\\', "\\\\").replace('"', "\\\"")),
                                InterpolatePart::Expr(e) => {
                                    let fmt = expr_type(e, ctx).as_ref().map(type_to_printf).unwrap_or("%ld");
                                    fmt_parts.push(fmt.to_string());
                                    expr_args.push(e.clone());
                                }
                            }
                        }
                        let fmt_str = fmt_parts.join("");
                        let suffix = if is_writeln || is_print { "\\n\"" } else { "\"" };
                        out.push_str(&format!("printf(\"{}{}", fmt_str, suffix));
                        for a in &expr_args {
                            out.push_str(", ");
                            emit_expr(out, a, ctx)?;
                        }
                        out.push_str(")");
                    } else {
                        let mut fmt_parts = Vec::new();
                        for arg in args {
                            let fmt = expr_type(arg, ctx)
                                .as_ref()
                                .map(type_to_printf)
                                .unwrap_or("%ld");
                            fmt_parts.push(fmt);
                        }
                        let fmt_str = fmt_parts.join(" ");
                        let suffix = if is_writeln || is_print { "\\n\"" } else { "\"" };
                        out.push_str(&format!("printf(\"{}{}", fmt_str, suffix));
                        for arg in args {
                            out.push_str(", ");
                            emit_expr(out, arg, ctx)?;
                        }
                        out.push_str(")");
                    }
                } else {
                    let mut fmt_parts = Vec::new();
                    for arg in args {
                        let fmt = expr_type(arg, ctx)
                            .as_ref()
                            .map(type_to_printf)
                            .unwrap_or("%ld");
                        fmt_parts.push(fmt);
                    }
                    let fmt_str = fmt_parts.join(" ");
                    let suffix = if is_writeln || is_print { "\\n\"" } else { "\"" };
                    out.push_str(&format!("printf(\"{}{}", fmt_str, suffix));
                    for arg in args {
                        out.push_str(", ");
                        emit_expr(out, arg, ctx)?;
                    }
                    out.push_str(")");
                }
            } else if is_read {
                out.push_str("({ int _r; scanf(\"%d\", &_r); (int32_t)_r; })");
            } else if is_strlen {
                if let Some(arg) = args.first() {
                    out.push_str("((size_t)strlen(");
                    emit_expr(out, arg, ctx)?;
                    out.push_str("))");
                }
            } else if is_panic {
                out.push_str("(fprintf(stderr, \"panic\\n\"), exit(1), (void)0)");
            } else if is_assert {
                if let Some(arg) = args.first() {
                    out.push_str("((");
                    emit_expr(out, arg, ctx)?;
                    out.push_str(") ? (void)0 : (fprintf(stderr, \"assertion failed\\n\"), exit(1)))");
                }
            } else if is_len {
                if let Some(arg) = args.first() {
                    if let Some(ty) = expr_type(arg, ctx) {
                        match ty {
                            Type::ArraySized(_, n) => out.push_str(&n.to_string()),
                            _ => {
                                out.push_str("(sizeof(");
                                emit_expr(out, arg, ctx)?;
                                out.push_str(") / sizeof((");
                                emit_expr(out, arg, ctx)?;
                                out.push_str(")[0]))");
                            }
                        }
                    } else {
                        out.push_str("(sizeof(");
                        emit_expr(out, arg, ctx)?;
                        out.push_str(") / sizeof((");
                        emit_expr(out, arg, ctx)?;
                        out.push_str(")[0]))");
                    }
                }
            } else if let Expr::Ident(name) = callee.as_ref() {
                let c_name = ctx.symbol_table.as_ref()
                    .map(|st| lookup_c_name(st, name))
                    .unwrap_or_else(|| name.clone());
                out.push_str(&c_name);
                out.push_str("(");
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        out.push_str(", ");
                    }
                    emit_expr(out, arg, ctx)?;
                }
                out.push_str(")");
            } else {
                emit_expr(out, callee, ctx)?;
                out.push_str("(");
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        out.push_str(", ");
                    }
                    emit_expr(out, arg, ctx)?;
                }
                out.push_str(")");
            }
        }
        Expr::Index { base, index } => {
            emit_expr(out, base, ctx)?;
            out.push_str("[");
            emit_expr(out, index, ctx)?;
            out.push_str("]");
        }
        Expr::Slice { base, start, end: _ } => {
            out.push_str("(&(");
            emit_expr(out, base, ctx)?;
            out.push_str(")[");
            if let Some(s) = start {
                emit_expr(out, s, ctx)?;
            } else {
                out.push_str("0");
            }
            out.push_str("])");
        }
        Expr::Field { base, field } => {
            if let Expr::Ident(mod_name) = base.as_ref() {
                if let Some(ref st) = ctx.symbol_table {
                    let is_module = st.scopes.iter().any(|s| s.modules.contains(mod_name));
                    if is_module {
                        out.push_str(&format!("{}_{}", mod_name, field));
                        return Ok(());
                    }
                }
            }
            emit_expr(out, base, ctx)?;
            out.push_str(&format!("->{}", field));
        }
        Expr::AddrOf(operand) => {
            out.push_str("&");
            emit_expr(out, operand, ctx)?;
        }
        Expr::Deref(operand) => {
            out.push_str("*");
            emit_expr(out, operand, ctx)?;
        }
        Expr::ArrayInit(elems) => {
            out.push_str("{ ");
            for (i, e) in elems.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                emit_expr(out, e, ctx)?;
            }
            out.push_str(" }");
        }
        Expr::Range { start, end } => {
            out.push_str("((");
            emit_expr(out, start, ctx)?;
            out.push_str(") <= (");
            emit_expr(out, end, ctx)?;
            out.push_str("))");
        }
        Expr::Tuple(elems) => {
            let mut tys = Vec::new();
            for e in elems {
                if let Some(t) = expr_type(e, ctx) {
                    tys.push(t);
                }
            }
            if tys.len() == elems.len() {
                let tuple_ty = Type::Tuple(tys);
                let c_ty = type_to_c_with_typedef(&tuple_ty, ctx);
                out.push_str(&format!("(({}) {{ ", c_ty));
                for (i, e) in elems.iter().enumerate() {
                    if i > 0 {
                        out.push_str(", ");
                    }
                    emit_expr(out, e, ctx)?;
                }
                out.push_str(" })");
            } else {
                out.push_str("(");
                for (i, e) in elems.iter().enumerate() {
                    if i > 0 {
                        out.push_str(", ");
                    }
                    emit_expr(out, e, ctx)?;
                }
                out.push_str(")");
            }
        }
        Expr::Interpolate(parts) => {
            let mut fmt_parts = Vec::new();
            let mut args = Vec::new();
            for p in parts {
                match p {
                    InterpolatePart::Str(s) => fmt_parts.push(s.replace('%', "%%").replace('\\', "\\\\").replace('"', "\\\"")),
                    InterpolatePart::Expr(e) => {
                        fmt_parts.push("%ld".to_string());
                        args.push(e.clone());
                    }
                }
            }
            out.push_str("(printf(\"");
            for fp in &fmt_parts {
                out.push_str(fp);
            }
            out.push_str("\\n\"");
            for a in &args {
                out.push_str(", ");
                emit_expr(out, a, ctx)?;
            }
            out.push_str("), (void)0)");
        }
        Expr::Cast { operand, target_ty } => {
            let c_ty = type_to_c(target_ty);
            out.push_str(&format!("(({})(", c_ty));
            emit_expr(out, operand, ctx)?;
            out.push_str("))");
        }
        Expr::New { class, args } => {
            out.push_str(&format!(
                "(({{ {}* _p = ({}*)malloc(sizeof({})); ",
                class, class, class
            ));
            if !args.is_empty() {
                out.push_str(&format!("{}_init(_p", class));
                for arg in args {
                    out.push_str(", ");
                    emit_expr(out, arg, ctx)?;
                }
                out.push_str("); ");
            }
            out.push_str("_p; }))");
        }
    }
    Ok(())
}
