// QuinusLang standard library: String utilities
// trim, concat (beyond +)

extern craft malloc(size: usize) -> link void;
extern craft ql_str_trim(s: str) -> str;
extern craft ql_str_concat(a: str, b: str) -> str;

realm str {
    craft trim(s: str) -> str {
        check (s == 0) {
            send "";
        }
        send ql_str_trim(s);
    }

    craft concat(a: str, b: str) -> str {
        check (a == 0) {
            send b;
        }
        check (b == 0) {
            send a;
        }
        send ql_str_concat(a, b);
    }
}
