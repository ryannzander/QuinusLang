/* Fibonacci benchmark: iterative fib(40) */
#include <stdio.h>
#include <stdint.h>

int64_t fib(int n) {
    if (n <= 1) return (int64_t)n;
    int64_t a = 0, b = 1;
    for (int i = 2; i <= n; i++) {
        int64_t tmp = a + b;
        a = b;
        b = tmp;
    }
    return b;
}

int main(void) {
    int64_t result = fib(40);
    printf("fib(40): %lld\n", (long long)result);
    return 0;
}
