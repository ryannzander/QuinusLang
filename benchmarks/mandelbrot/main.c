/* Mandelbrot set: render small grid, count iterations */
#include <stdio.h>
#include <stdint.h>

int main(void) {
    const int w = 160, h = 120;
    int64_t total = 0;
    for (int y = 0; y < h; y++) {
        for (int x = 0; x < w; x++) {
            double cr = (double)x / (double)w * 2.5 - 1.5;
            double ci = (double)y / (double)h * 2.0 - 1.0;
            double zr = 0.0, zi = 0.0;
            int n = 0;
            while (n < 100) {
                double zr2 = zr * zr, zi2 = zi * zi;
                if (zr2 + zi2 > 4.0) break;
                double new_zi = 2.0 * zr * zi + ci;
                zr = zr2 - zi2 + cr;
                zi = new_zi;
                n++;
            }
            total += (int64_t)n;
        }
    }
    printf("mandelbrot: %lld\n", (long long)total);
    return 0;
}
