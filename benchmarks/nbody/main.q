// N-body simulation: classic 5-body, ~50k iterations
extern craft sqrt(x: f64) -> f64;

craft main() -> void {
    make shift x: [f64; 5] = { 0.0, 1.0, -1.0, 0.0, 0.0 };
    make shift y: [f64; 5] = { 0.0, 0.0, 0.0, 1.0, -1.0 };
    make shift z: [f64; 5] = { 0.0, 0.0, 0.0, 0.0, 0.0 };
    make shift vx: [f64; 5] = { 0.0, 0.0, 0.0, -1.0, 1.0 };
    make shift vy: [f64; 5] = { 0.0, 1.0, -1.0, 0.0, 0.0 };
    make shift vz: [f64; 5] = { 0.0, 0.0, 0.0, 0.0, 0.0 };
    make mass: [f64; 5] = { 1.0, 0.000001, 0.000001, 0.000001, 0.000001 };
    make dt: f64 = 0.01;
    make shift step: i32 = 0;
    loopwhile (step < 50000) {
        make shift i: i32 = 0;
        loopwhile (i < 5) {
            make shift j: i32 = i + 1;
            loopwhile (j < 5) {
                make dx: f64 = x[i] - x[j];
                make dy: f64 = y[i] - y[j];
                make dz: f64 = z[i] - z[j];
                make dist2: f64 = dx * dx + dy * dy + dz * dz;
                make dist: f64 = sqrt(dist2);
                make mag: f64 = dt / (dist * dist * dist);
                make mi: f64 = mass[i];
                make mj: f64 = mass[j];
                vx[i] = vx[i] - dx * mj * mag;
                vy[i] = vy[i] - dy * mj * mag;
                vz[i] = vz[i] - dz * mj * mag;
                vx[j] = vx[j] + dx * mi * mag;
                vy[j] = vy[j] + dy * mi * mag;
                vz[j] = vz[j] + dz * mi * mag;
                j = j + 1;
            }
            x[i] = x[i] + vx[i] * dt;
            y[i] = y[i] + vy[i] * dt;
            z[i] = z[i] + vz[i] * dt;
            i = i + 1;
        }
        step = step + 1;
    }
    make e: f64 = x[0] + y[0] + z[0];
    print(`nbody: ${e}`);
    send;
}
