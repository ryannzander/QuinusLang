// Mandelbrot set: render small grid, count iterations
craft main() -> void {
    make w: i32 = 160;
    make h: i32 = 120;
    make wf: f64 = w as f64;
    make hf: f64 = h as f64;
    make c2_5: f64 = 2.5;
    make c1_5: f64 = 1.5;
    make c1_0: f64 = 1.0;
    make c2_0: f64 = 2.0;
    make c4_0: f64 = 4.0;
    make shift total: i64 = 0;
    make shift y: i32 = 0;
    loopwhile (y < h) {
        make shift x: i32 = 0;
        loopwhile (x < w) {
            make xf: f64 = x as f64;
            make yf: f64 = y as f64;
            make cr: f64 = xf / wf * c2_5 - c1_5;
            make ci: f64 = yf / hf * c2_0 - c1_0;
            make shift zr: f64 = 0.0;
            make shift zi: f64 = 0.0;
            make shift n: i32 = 0;
            loopwhile (n < 100) {
                make zr2: f64 = zr * zr;
                make zi2: f64 = zi * zi;
                check (zr2 + zi2 > c4_0) {
                    stop;
                }
                make new_zi: f64 = c2_0 * zr * zi + ci;
                zr = zr2 - zi2 + cr;
                zi = new_zi;
                n = n + 1;
            }
            total = total + (n as i64);
            x = x + 1;
        }
        y = y + 1;
    }
    print(`mandelbrot: ${total}`);
    send;
}
