#include "placement_new.h"
#include <iostream>
#include <new>

int main() {
    alignas(Vec3) unsigned char buf[sizeof(Vec3)];
    vec3_construct(buf, 1.0f, 2.0f, 3.0f);
    Vec3* v = reinterpret_cast<Vec3*>(buf);
    std::cout << "v = (" << vec3_x(v) << "," << vec3_y(v) << "," << vec3_z(v) << ")\n";
    vec3_destruct(v);
    return 0;
}
