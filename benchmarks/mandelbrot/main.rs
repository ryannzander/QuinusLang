// Mandelbrot set: render small grid, count iterations
fn main() {
    let w: i32 = 160;
    let h: i32 = 120;
    let mut total: i64 = 0;
    for y in 0..h {
        for x in 0..w {
            let cr = (x as f64) / (w as f64) * 2.5 - 1.5;
            let ci = (y as f64) / (h as f64) * 2.0 - 1.0;
            let mut zr: f64 = 0.0;
            let mut zi: f64 = 0.0;
            let mut n: i32 = 0;
            while n < 100 {
                let zr2 = zr * zr;
                let zi2 = zi * zi;
                if zr2 + zi2 > 4.0 {
                    break;
                }
                let new_zi = 2.0 * zr * zi + ci;
                zr = zr2 - zi2 + cr;
                zi = new_zi;
                n += 1;
            }
            total += n as i64;
        }
    }
    println!("mandelbrot: {}", total);
}
