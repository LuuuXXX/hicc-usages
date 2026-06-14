#pragma once

#include <cstddef>

// Placement new: construct an object in a pre-allocated buffer. Rust side
// owns the buffer (Vec<u8>); C++ constructs/destroys in place.

struct Vec3 {
    float x, y, z;
};

// Returns size/alignment requirements.
constexpr std::size_t vec3_size()      { return sizeof(Vec3); }
constexpr std::size_t vec3_align()     { return alignof(Vec3); }

// Construct a Vec3 inside the given buffer (must be >= vec3_size bytes).
void vec3_construct(void* buf, float x, float y, float z);
// Destroy in place.
void vec3_destruct(Vec3* v);
// Getters — pass Vec3 pointer.
float vec3_x(const Vec3* v);
float vec3_y(const Vec3* v);
float vec3_z(const Vec3* v);
