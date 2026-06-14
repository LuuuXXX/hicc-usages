//! 031_custom_deleter: unique_ptr 自定义 deleter
//!
//! hicc 模式：`unique_ptr<int[], ArrayDeleter>` 不能直接跨 FFI（数组类型，非 class）。
//! 在 C++ 侧用 `IntArrayHandle { IntArrayPtr ptr; size_t size; }` 包装，
//! 再在 cpp! 块里用 `void*` 转接，避免 hicc 把 `IntArrayHandle*` 当作需要注册的 class。

use std::ffi::c_void;

#[repr(C)]
pub struct IntArrayHandle {
    _private: [u8; 0],
}

hicc::cpp! {
    #include "custom_deleter.h"

    inline void* cd_make_handle(size_t n) {
        return custom_deleter_ns::make_int_array_handle(n);
    }
    inline int cd_handle_read(void* h, size_t i) {
        return custom_deleter_ns::handle_read_at(
            reinterpret_cast<const custom_deleter_ns::IntArrayHandle*>(h), i);
    }
    inline size_t cd_handle_size(void* h) {
        return custom_deleter_ns::handle_size(
            reinterpret_cast<const custom_deleter_ns::IntArrayHandle*>(h));
    }
    inline void cd_destroy_handle(void* h) {
        custom_deleter_ns::destroy_int_array_handle(
            reinterpret_cast<custom_deleter_ns::IntArrayHandle*>(h));
    }
}

hicc::import_lib! {
    #![link_name = "custom_deleter"]

    #[cpp(func = "void* cd_make_handle(size_t)")]
    pub fn make_int_array_handle(n: usize) -> *mut c_void;

    #[cpp(func = "int cd_handle_read(void*, size_t)")]
    pub fn handle_read_at(h: *mut c_void, i: usize) -> i32;

    #[cpp(func = "size_t cd_handle_size(void*)")]
    pub fn handle_size(h: *mut c_void) -> usize;

    #[cpp(func = "void cd_destroy_handle(void*)")]
    pub fn destroy_int_array_handle(h: *mut c_void);

    #[cpp(func = "int custom_deleter_ns::custom_deleter_status()")]
    pub fn custom_deleter_status() -> i32;
}

/// RAII wrapper: 在 Drop 中调 destroy_int_array_handle
pub struct IntArray {
    raw: *mut c_void,
}

impl IntArray {
    pub fn new(n: usize) -> Self {
        Self { raw: unsafe { make_int_array_handle(n) } }
    }
    pub fn read_at(&self, i: usize) -> i32 {
        unsafe { handle_read_at(self.raw, i) }
    }
    pub fn size(&self) -> usize {
        unsafe { handle_size(self.raw) }
    }
}

impl Drop for IntArray {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe { destroy_int_array_handle(self.raw) };
            self.raw = std::ptr::null_mut();
        }
    }
}

#[allow(dead_code)]
fn _link_type(_: *mut IntArrayHandle) {}
