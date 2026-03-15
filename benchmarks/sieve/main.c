/* Sieve of Eratosthenes: find primes up to N */
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <math.h>
#include <string.h>

int main(void) {
    const size_t N = 1000000;
    uint8_t *buf = (uint8_t *)malloc(N + 1);
    memset(buf, 1, N + 1);
    size_t limit = (size_t)sqrt((double)N);
    for (size_t i = 2; i <= limit; i++) {
        if (buf[i]) {
            for (size_t j = i * i; j <= N; j += i) {
                buf[j] = 0;
            }
        }
    }
    size_t count = 0;
    for (size_t k = 2; k <= N; k++) {
        if (buf[k]) count++;
    }
    free(buf);
    printf("primes up to %zu: %zu\n", N, count);
    return 0;
}
