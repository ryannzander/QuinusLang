// QuinusLang standard library: Path manipulation
// Simple path utilities using str.concat

bring "str";

realm path {
    craft join(a: str, b: str) -> str {
        check (a == 0) {
            send b;
        }
        check (b == 0) {
            send a;
        }
        make sep: str = "/";
        send str.concat(str.concat(a, sep), b);
    }
}
