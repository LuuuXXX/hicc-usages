#![no_std]
#![feature(specialization, lang_items)]

extern crate alloc;

use alloc::alloc::{GlobalAlloc, Layout};
use alloc::collections::BTreeMap;
use core::panic::PanicInfo;
use core::sync::atomic::{AtomicUsize, Ordering};

use hicc_rs::{export_class, export_lib};

// ===== Bump allocator =====

const HEAP_SIZE: usize = 65536;
static mut HEAP: [u8; HEAP_SIZE] = [0; HEAP_SIZE];
static HEAP_POS: AtomicUsize = AtomicUsize::new(0);

pub struct BumpAlloc;

unsafe impl GlobalAlloc for BumpAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        loop {
            let pos = HEAP_POS.load(Ordering::Relaxed);
            let aligned = (pos + layout.align() - 1) & !(layout.align() - 1);
            if aligned + layout.size() > HEAP_SIZE {
                return core::ptr::null_mut();
            }
            if HEAP_POS
                .compare_exchange_weak(
                    pos,
                    aligned + layout.size(),
                    Ordering::SeqCst,
                    Ordering::SeqCst,
                )
                .is_ok()
            {
                return unsafe { HEAP.as_mut_ptr().add(aligned) };
            }
        }
    }
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

#[global_allocator]
static ALLOC: BumpAlloc = BumpAlloc;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[lang = "eh_personality"]
fn eh_personality() {}

// ===== Custom types =====

pub struct Container<T>(pub T);

impl<T> Container<T> {
    fn get(&self) -> &T {
        &self.0
    }
}

#[export_class]
mod classes {
    impl<T> super::Container<T> {
        fn get(&self) -> &T;
    }
}

// ===== Function implementations =====

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn negate(x: i32) -> i32 {
    -x
}

fn container_value(x: Container<i32>) -> i32 {
    *x.get()
}

fn new_container(x: i32) -> Container<i32> {
    Container(x)
}

fn double_option(x: Option<i32>) -> i64 {
    match x {
        Some(v) => (v as i64) * 2,
        None => -1,
    }
}

fn new_option(x: i32) -> Option<i32> {
    Some(x)
}

fn btreemap_demo() -> i32 {
    let mut map = BTreeMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    let mut sum = 0;
    for (&k, &v) in map.iter() {
        sum += k + v;
    }
    sum
}

// ===== Library =====

#[export_lib(name = "no_std_demo")]
mod lib {
    use super::*;
    fn add(x: i32, y: i32) -> i32;
    fn negate(x: i32) -> i32;
    fn container_value(x: Container<i32>) -> i32;
    fn new_container(x: i32) -> Container<i32>;
    fn double_option(x: Option<i32>) -> i64;
    fn new_option(x: i32) -> Option<i32>;
    fn btreemap_demo() -> i32;
}
