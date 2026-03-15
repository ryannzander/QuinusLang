/* Sum benchmark: simple loop summing 1..N */
#include <stdio.h>
#include <stdint.h>

int main(void) {
    int64_t N = 100000000;
    int64_t s = 0;
    for (int64_t i = 0; i < N; i++) {
        s += i;
    }
    printf("sum: %lld\n", (long long)s);
    return 0;
}
