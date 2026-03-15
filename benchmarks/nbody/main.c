/* N-body simulation: classic 5-body, ~50k iterations */
#include <stdio.h>
#include <math.h>

typedef struct {
    double x, y, z, vx, vy, vz, mass;
} Body;

int main(void) {
    Body bodies[5] = {
        { 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0 },
        { 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1e-6 },
        { -1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 1e-6 },
        { 0.0, 1.0, 0.0, -1.0, 0.0, 0.0, 1e-6 },
        { 0.0, -1.0, 0.0, 1.0, 0.0, 0.0, 1e-6 }
    };
    const double dt = 0.01;
    for (int step = 0; step < 50000; step++) {
        for (int i = 0; i < 5; i++) {
            for (int j = i + 1; j < 5; j++) {
                double dx = bodies[i].x - bodies[j].x;
                double dy = bodies[i].y - bodies[j].y;
                double dz = bodies[i].z - bodies[j].z;
                double dist2 = dx * dx + dy * dy + dz * dz;
                double dist = sqrt(dist2);
                double mag = dt / (dist * dist * dist);
                double mi = bodies[i].mass, mj = bodies[j].mass;
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
    double e = bodies[0].x + bodies[0].y + bodies[0].z;
    printf("nbody: %f\n", e);
    return 0;
}
