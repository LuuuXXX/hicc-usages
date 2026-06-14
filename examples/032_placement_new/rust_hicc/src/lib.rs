//! 032_placement_new: placement new + 显式析构
//!
//! hicc 模式：Buffer 通过 hicc::make_unique 工厂构造（普通类），用 `using` 别名屏蔽命名空间。
//! Payload 通过 cpp! 块用 void* 屏蔽 — place_payload_raw 返回 void*，
//! Rust 端持有裸指针，destroy_payload 显式调 C++ 析构。

hicc::cpp! {
    #include "placement_new.h"
    #include <hicc/std/memory.hpp>

    using BufferH = placement_new_ns::Buffer;

    inline void* cd_place(void* raw, int v) {
        return placement_new_ns::place_payload_raw(raw, v);
    }
    inline int cd_value(void* p) {
        return static_cast<placement_new_ns::Payload*>(p)->value();
    }
    inline void cd_set(void* p, int v) {
        static_cast<placement_new_ns::Payload*>(p)->set(v);
    }
    inline void cd_destroy(void* p) {
        placement_new_ns::destroy_payload(static_cast<placement_new_ns::Payload*>(p));
    }
}

hicc::import_class! {
    #[cpp(class = "BufferH")]
    pub class Buffer {
        #[cpp(method = "size_t size() const")]
        pub fn size(&self) -> usize;

        pub fn new(sz: usize) -> Self { buffer_new(sz) }
    }
}

hicc::import_lib! {
    #![link_name = "placement_new"]

    #[cpp(func = "std::unique_ptr<BufferH> hicc::make_unique<BufferH, size_t>(size_t&&)")]
    pub fn buffer_new(sz: usize) -> Buffer;

    #[cpp(func = "void* cd_place(void*, int)")]
    pub fn place_payload_raw(raw: *mut std::ffi::c_void, value: i32) -> *mut std::ffi::c_void;

    #[cpp(func = "int cd_value(void*)")]
    pub fn payload_value(p: *mut std::ffi::c_void) -> i32;

    #[cpp(func = "void cd_set(void*, int)")]
    pub fn payload_set(p: *mut std::ffi::c_void, v: i32);

    #[cpp(func = "void cd_destroy(void*)")]
    pub fn destroy_payload(p: *mut std::ffi::c_void);
}

/// 在 Rust 端 buffer 上构造 Payload；Drop 时显式调析构。
pub struct Placement {
    payload: *mut std::ffi::c_void,
    destroyed: bool,
}

impl Placement {
    pub fn new(buf: &mut [u8], value: i32) -> Self {
        let p = place_payload_raw(buf.as_mut_ptr() as *mut std::ffi::c_void, value);
        Self { payload: p, destroyed: false }
    }
    pub fn value(&self) -> i32 {
        payload_value(self.payload)
    }
    pub fn set(&mut self, v: i32) {
        payload_set(self.payload, v);
    }
}

impl Drop for Placement {
    fn drop(&mut self) {
        if !self.destroyed {
            destroy_payload(self.payload);
            self.destroyed = true;
        }
    }
}
