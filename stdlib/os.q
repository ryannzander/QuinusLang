// QuinusLang standard library: Process execution and environment
// Uses C FFI to wrap system(), getenv

extern craft system(cmd: str) -> i32;
extern craft getenv(name: str) -> str;

realm os {
    craft run(cmd: str) -> i32 {
        send system(cmd);
    }

    craft getenv(name: str) -> str {
        make p: str = getenv(name);
        check (p == 0) {
            send "";
        }
        send p;
    }
}
