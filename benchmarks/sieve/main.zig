// Sieve of Eratosthenes: find primes up to N
const std = @import("std");

pub fn main() !void {
    const n: usize = 1_000_000;
    var buf = try std.heap.page_allocator.alloc(u8, n + 1);
    defer std.heap.page_allocator.free(buf);
    @memset(buf, 1);
    const limit = @as(usize, @intFromFloat(std.math.sqrt(@as(f64, @floatFromInt(n)))));
    var i: usize = 2;
    while (i <= limit) : (i += 1) {
        if (buf[i] != 0) {
            var j = i * i;
            while (j <= n) : (j += i) {
                buf[j] = 0;
            }
        }
    }
    var count: usize = 0;
    var k: usize = 2;
    while (k <= n) : (k += 1) {
        if (buf[k] != 0) count += 1;
    }
    std.debug.print("primes up to {}: {}\n", .{ n, count });
}
