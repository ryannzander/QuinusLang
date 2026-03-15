// Sum benchmark: simple loop summing 1..N
const std = @import("std");

pub fn main() !void {
    const n: i64 = 100_000_000;
    var s: i64 = 0;
    var i: i64 = 0;
    while (i < n) : (i += 1) {
        s += i;
    }
    std.debug.print("sum: {}\n", .{s});
}
