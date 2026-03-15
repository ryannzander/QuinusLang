// Mandelbrot set: render small grid, count iterations
const std = @import("std");

pub fn main() !void {
    const w: i32 = 160;
    const h: i32 = 120;
    var total: i64 = 0;
    var y: i32 = 0;
    while (y < h) : (y += 1) {
        var x: i32 = 0;
        while (x < w) : (x += 1) {
            const cr = @as(f64, @floatFromInt(x)) / @as(f64, @floatFromInt(w)) * 2.5 - 1.5;
            const ci = @as(f64, @floatFromInt(y)) / @as(f64, @floatFromInt(h)) * 2.0 - 1.0;
            var zr: f64 = 0.0;
            var zi: f64 = 0.0;
            var n: i32 = 0;
            while (n < 100) : (n += 1) {
                const zr2 = zr * zr;
                const zi2 = zi * zi;
                if (zr2 + zi2 > 4.0) break;
                const new_zi = 2.0 * zr * zi + ci;
                zr = zr2 - zi2 + cr;
                zi = new_zi;
            }
            total += @intCast(n);
        }
    }
    std.debug.print("mandelbrot: {}\n", .{total});
}
