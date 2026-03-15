// N-body simulation: classic 5-body, ~50k iterations
const std = @import("std");

const Body = struct {
    x: f64, y: f64, z: f64,
    vx: f64, vy: f64, vz: f64,
    mass: f64,
};

pub fn main() !void {
    var bodies = [5]Body{
        .{ .x = 0.0, .y = 0.0, .z = 0.0, .vx = 0.0, .vy = 0.0, .vz = 0.0, .mass = 1.0 },
        .{ .x = 1.0, .y = 0.0, .z = 0.0, .vx = 0.0, .vy = 1.0, .vz = 0.0, .mass = 1e-6 },
        .{ .x = -1.0, .y = 0.0, .z = 0.0, .vx = 0.0, .vy = -1.0, .vz = 0.0, .mass = 1e-6 },
        .{ .x = 0.0, .y = 1.0, .z = 0.0, .vx = -1.0, .vy = 0.0, .vz = 0.0, .mass = 1e-6 },
        .{ .x = 0.0, .y = -1.0, .z = 0.0, .vx = 1.0, .vy = 0.0, .vz = 0.0, .mass = 1e-6 },
    };
    const dt: f64 = 0.01;
    var step: i32 = 0;
    while (step < 50000) : (step += 1) {
        var i: usize = 0;
        while (i < 5) : (i += 1) {
            var j = i + 1;
            while (j < 5) : (j += 1) {
                const dx = bodies[i].x - bodies[j].x;
                const dy = bodies[i].y - bodies[j].y;
                const dz = bodies[i].z - bodies[j].z;
                const dist2 = dx * dx + dy * dy + dz * dz;
                const dist = @sqrt(dist2);
                const mag = dt / (dist * dist * dist);
                const mi = bodies[i].mass;
                const mj = bodies[j].mass;
                bodies[i].vx -= dx * mj * mag;
                bodies[i].vy -= dy * mj * mag;
                bodies[i].vz -= dz * mj * mag;
                bodies[j].vx += dx * mi * mag;
                bodies[j].vy += dy * mi * mag;
                bodies[j].vz += dz * mi * mag;
            }
            bodies[i].x += bodies[i].vx * dt;
            bodies[i].y += bodies[i].vy * dt;
            bodies[i].z += bodies[i].vz * dt;
        }
    }
    const e = bodies[0].x + bodies[0].y + bodies[0].z;
    std.debug.print("nbody: {}\n", .{e});
}
