#include "placement_new.h"
#include <new>

void vec3_construct(void* buf, float x, float y, float z) {
    new (buf) Vec3{x, y, z};
}

void vec3_destruct(Vec3* v) {
    // No dynamic resources; trivially destructible. The explicit destructor
    // call is for demonstration.
    v->~Vec3();
}

float vec3_x(const Vec3* v) { return v->x; }
float vec3_y(const Vec3* v) { return v->y; }
float vec3_z(const Vec3* v) { return v->z; }
