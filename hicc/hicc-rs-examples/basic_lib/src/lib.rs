#![feature(specialization)]

use hicc_rs::{export_class, export_lib};

// ===== Custom types =====
pub struct Container<T>(pub T);

impl<T> Container<T> {
    fn get(&self) -> &T {
        &self.0
    }
}

pub mod conflict {
    pub struct Container<T>(pub T);

    impl<T> Container<T> {
        pub fn get(&self) -> &T {
            &self.0
        }
    }
}

// ===== Batch export_class for both Container types =====
#[export_class]
mod classes {
    impl<T> super::Container<T> {
        fn get(&self) -> &T;
    }

    impl<T> super::conflict::Container<T> {
        fn get(&self) -> &T;
    }
}

// ===== Pointer<T>: sync method body using generic Self — Self as constructor =====
pub struct Pointer<T>(pub T);

impl<T> Pointer<T> {
    fn get(&self) -> &T {
        &self.0
    }
}

#[export_class]
impl<T> Pointer<T> {
    fn get(&self) -> &T;
    fn rewrap(self) -> Self {
        Self(self.0)
    }
}

fn new_pointer(val: i32) -> Pointer<i32> {
    Pointer(val)
}

// ===== Plain #[repr(C)] struct (no export_class) for cbindgen testing =====
#[repr(C)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

// Static data backing the factory functions below.
static HICC_SLICE_DATA: [Option<i32>; 3] = [Some(10), None, Some(30)];
static HICC_ARR_DATA: [&'static str; 3] = ["a", "bb", "ccc"];

// ===== Function implementations (outside the mod block) =====

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn negate(x: i32) -> i32 {
    -x
}

fn container_value(x: Container<i32>) -> i32 {
    *x.get()
}

fn conflict_value(x: conflict::Container<i32>) -> i32 {
    *x.get()
}

fn double_option(x: Option<i32>) -> i64 {
    match x {
        Some(v) => (v as i64) * 2,
        None => -1,
    }
}

fn check_str(s: &'static str) -> usize {
    s.len()
}

fn count_some(s: &'static [Option<i32>]) -> usize {
    s.iter().filter_map(|x| *x).count()
}

fn total_len(arr: [&'static str; 3]) -> usize {
    arr.iter().map(|s| s.len()).sum()
}

fn add_point(p: Point, q: Point) -> Point {
    Point {
        x: p.x + q.x,
        y: p.y + q.y,
    }
}

fn new_container(x: i32) -> Container<i32> {
    Container(x)
}

fn new_option(x: i32) -> Option<i32> {
    Some(x)
}

fn new_str() -> &'static str {
    "hello"
}

fn new_slice() -> &'static [Option<i32>] {
    &HICC_SLICE_DATA
}

fn new_array() -> [&'static str; 3] {
    HICC_ARR_DATA
}

// ===== Library - all hicc-rs supported types as parameters =====
#[export_lib(name = "demo")]
mod lib {
    use super::*;
    fn add(x: i32, y: i32) -> i32;
    fn negate(x: i32) -> i32;
    fn container_value(x: Container<i32>) -> i32;
    fn conflict_value(x: conflict::Container<i32>) -> i32;
    fn double_option(x: Option<i32>) -> i64;
    fn check_str(s: &'static str) -> usize;
    fn count_some(s: &'static [Option<i32>]) -> usize;
    fn total_len(arr: [&'static str; 3]) -> usize;
    fn add_point(p: Point, q: Point) -> Point;
    fn new_container(x: i32) -> Container<i32>;
    fn new_option(x: i32) -> Option<i32>;
    fn new_pointer(val: i32) -> Pointer<i32>;
    fn new_str() -> &'static str;
    fn new_slice() -> &'static [Option<i32>];
    fn new_array() -> [&'static str; 3];
}
