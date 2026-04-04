// Q++ standard library: Process execution and environment
// Uses C FFI to wrap system(), getenv, getcwd

extern craft system(cmd: str) -> i32;
extern craft getenv(name: str) -> str;
extern craft getcwd(buf: str, size: usize) -> str;
extern craft malloc(size: usize) -> link void;

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

    craft cwd() -> str {
        make buf: link void = malloc(4096);
        make p: str = getcwd(buf, 4096);
        check (p == 0) {
            send "";
        }
        send p;
    }
}
