// QuinusLang standard library: Random numbers
// Uses C FFI: stdlib.h (rand, srand)

extern craft rand() -> i32;
extern craft srand(seed: u32) -> void;

realm rand {
    craft next() -> i32 {
        send rand();
    }

    craft seed(s: u32) -> void {
        srand(s);
        send;
    }
}
