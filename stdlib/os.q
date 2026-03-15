// QuinusLang standard library: Process execution
// Uses C FFI to wrap system()

extern craft system(cmd: str) -> i32;

realm os {
    craft run(cmd: str) -> i32 {
        send system(cmd);
    }
}
