// Fibonacci benchmark: iterative fib(40)
const std = @import("std");

fn fib(n: i32) i64 {
    if (n <= 1) return @intCast(n);
    var a: i64 = 0;
    var b: i64 = 1;
    var i: i32 = 2;
    while (i <= n) : (i += 1) {
        const tmp = a + b;
        a = b;
        b = tmp;
    }
    return b;
}

pub fn main() !void {
    const result = fib(40);
    std.debug.print("fib(40): {}\n", .{result});
}
