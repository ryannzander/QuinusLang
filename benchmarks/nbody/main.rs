// N-body simulation: classic 5-body, ~50k iterations
struct Body {
    x: f64, y: f64, z: f64,
    vx: f64, vy: f64, vz: f64,
    mass: f64,
}

fn main() {
    let mut bodies = [
        Body { x: 0.0, y: 0.0, z: 0.0, vx: 0.0, vy: 0.0, vz: 0.0, mass: 1.0 },
        Body { x: 1.0, y: 0.0, z: 0.0, vx: 0.0, vy: 1.0, vz: 0.0, mass: 1e-6 },
        Body { x: -1.0, y: 0.0, z: 0.0, vx: 0.0, vy: -1.0, vz: 0.0, mass: 1e-6 },
        Body { x: 0.0, y: 1.0, z: 0.0, vx: -1.0, vy: 0.0, vz: 0.0, mass: 1e-6 },
        Body { x: 0.0, y: -1.0, z: 0.0, vx: 1.0, vy: 0.0, vz: 0.0, mass: 1e-6 },
    ];
    let dt = 0.01;
    for _step in 0..50000 {
        for i in 0..5 {
            for j in (i + 1)..5 {
                let dx = bodies[i].x - bodies[j].x;
                let dy = bodies[i].y - bodies[j].y;
                let dz = bodies[i].z - bodies[j].z;
                let dist2 = dx * dx + dy * dy + dz * dz;
                let dist = dist2.sqrt();
                let mag = dt / (dist * dist * dist);
                let mi = bodies[i].mass;
                let mj = bodies[j].mass;
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
    let e = bodies[0].x + bodies[0].y + bodies[0].z;
    println!("nbody: {}", e);
}
