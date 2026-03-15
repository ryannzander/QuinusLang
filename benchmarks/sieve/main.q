// Sieve of Eratosthenes: find primes up to N
bring "std.mem";

extern craft malloc(size: usize) -> link void;
extern craft free(ptr: link void) -> void;
extern craft sqrt(x: f64) -> f64;

craft main() -> void {
    make N: usize = 1_000_000;
    make buf: link u8 = malloc(N + 1) as link u8;
    mem.set(buf, 1, N + 1);
    make shift i: usize = 2;
    make limit: usize = sqrt(N as f64) as usize;
    loopwhile (i <= limit) {
        check (buf[i] != 0) {
            make shift j: usize = i * i;
            loopwhile (j <= N) {
                buf[j] = 0;
                j = j + i;
            }
        }
        i = i + 1;
    }
    make shift count: usize = 0;
    make shift k: usize = 2;
    loopwhile (k <= N) {
        check (buf[k] != 0) {
            count = count + 1;
        }
        k = k + 1;
    }
    free(buf as link void);
    print(`primes up to ${N}: ${count}`);
    send;
}
