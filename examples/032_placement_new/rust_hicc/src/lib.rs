// Placement new: Rust owns the buffer, C++ constructs/destroys in place.
// We expose size/align so Rust can allocate a properly-aligned buffer.

hicc::cpp! {
    #include "placement_new.h"
}

hicc::import_lib! {
    #![link_name = "placement_new_hicc"]

    #[cpp(func = "std::size_t vec3_size()")]
    pub fn vec3_size() -> usize;

    #[cpp(func = "std::size_t vec3_align()")]
    pub fn vec3_align() -> usize;

    #[cpp(func = "void vec3_construct(void*, float, float, float)")]
    pub fn vec3_construct(buf: *mut u8, x: f32, y: f32, z: f32);

    #[cpp(func = "void vec3_destruct(Vec3*)")]
    pub fn vec3_destruct(v: *mut u8);

    #[cpp(func = "float vec3_x(const Vec3*)")]
    pub fn vec3_x(v: *const u8) -> f32;

    #[cpp(func = "float vec3_y(const Vec3*)")]
    pub fn vec3_y(v: *const u8) -> f32;

    #[cpp(func = "float vec3_z(const Vec3*)")]
    pub fn vec3_z(v: *const u8) -> f32;
}
