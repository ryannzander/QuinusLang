// QuinusLang standard library: Memory utilities
// Thin wrappers over string.h

extern craft memcpy(dest: link void, src: link void, n: usize) -> link void;
extern craft memset(ptr: link void, c: i32, n: usize) -> link void;
extern craft memcmp(a: link void, b: link void, n: usize) -> i32;

realm mem {
    craft copy(dest: link void, src: link void, n: usize) -> link void {
        send memcpy(dest, src, n);
    }

    craft set(ptr: link void, c: i32, n: usize) -> link void {
        send memset(ptr, c, n);
    }

    craft compare(a: link void, b: link void, n: usize) -> i32 {
        send memcmp(a, b, n);
    }
}
