#ifndef RUNTIME_H
#define RUNTIME_H

#include <stdlib.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include <math.h>

#ifdef _WIN32
#include <direct.h>
#define getcwd _getcwd
#else
#include <unistd.h>
#endif

/* Result type for checked arithmetic */
typedef struct {
    int is_ok;
    union { int32_t val; int32_t err; } u;
} ql_result_i32_i32;

/* Vec types (used by vec, map, lexer) */
typedef struct { void** data; size_t len; size_t cap; } ql_vec_ptr_t;
typedef struct { char* data; size_t len; size_t cap; } ql_vec_u8_t;

/* Map pair (used by map) */
typedef struct { char* key; void* value; } ql_map_pair_t;

/* Token (used by lexer) */
typedef struct { int ty; size_t line; size_t col; char* str_val; long int_val; } ql_token_t;

/* AST expr (used by ast) */
typedef struct { int tag; long int_val; char* str_val; void* left; void* right; void* args; } ast_Expr_t;

#endif
