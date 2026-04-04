// Q++ standard library: String formatting (sprintf-style)
// For codegen string building

extern craft malloc(size: usize) -> link void;
extern craft snprintf(buf: str, size: usize, fmt: str, a: i64) -> i32;
extern craft ql_fmt_sprintf_s(buf: str, size: usize, fmt: str, s: str) -> i32;
extern craft ql_fmt_sprintf_ii(buf: str, size: usize, fmt: str, a: i64, b: i64) -> i32;
extern craft ql_fmt_sprintf_si(buf: str, size: usize, fmt: str, s: str, a: i64) -> i32;
extern craft ql_fmt_sprintf_ss(buf: str, size: usize, fmt: str, a: str, b: str) -> i32;
extern craft ql_fmt_alloc_i(fmt: str, a: i64) -> str;
extern craft ql_fmt_alloc_s(fmt: str, s: str) -> str;
extern craft ql_fmt_alloc_si(fmt: str, s: str, a: i64) -> str;

realm fmt {
    craft sprintf_i(buf: str, size: usize, fmt: str, a: i64) -> i32 {
        send snprintf(buf, size, fmt, a);
    }

    craft sprintf_s(buf: str, size: usize, fmt: str, s: str) -> i32 {
        send ql_fmt_sprintf_s(buf, size, fmt, s);
    }

    craft sprintf_ii(buf: str, size: usize, fmt: str, a: i64, b: i64) -> i32 {
        send ql_fmt_sprintf_ii(buf, size, fmt, a, b);
    }

    craft sprintf_si(buf: str, size: usize, fmt: str, s: str, a: i64) -> i32 {
        send ql_fmt_sprintf_si(buf, size, fmt, s, a);
    }

    craft sprintf_ss(buf: str, size: usize, fmt: str, a: str, b: str) -> i32 {
        send ql_fmt_sprintf_ss(buf, size, fmt, a, b);
    }

    craft alloc_i(fmt: str, a: i64) -> str {
        send ql_fmt_alloc_i(fmt, a);
    }

    craft alloc_s(fmt: str, s: str) -> str {
        send ql_fmt_alloc_s(fmt, s);
    }

    craft alloc_si(fmt: str, s: str, a: i64) -> str {
        send ql_fmt_alloc_si(fmt, s, a);
    }
}
