#![feature(specialization)]

use hicc_rs::*;

// ---- Basic type alias ----
pub type MyInt = i32;

// ---- POD struct ----
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

// ---- POD type alias ----
pub type MyPoint = Point;

// ---- POD using aliased types in fields ----
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Line {
    pub start: MyPoint,
    pub end: MyPoint,
}

// ---- Non-POD struct ----
pub struct Counter {
    count: i64,
}

impl Counter {
    pub fn new(initial: i64) -> Self {
        Counter { count: initial }
    }

    pub fn get(&self) -> i64 {
        self.count
    }

    pub fn increment(&mut self, delta: i64) {
        self.count += delta;
    }
}

// ---- Non-POD type alias ----
pub type MyCounter = Counter;

// ---- Another non-POD using basic type alias ----
pub struct Accumulator {
    total: i64,
}

impl Accumulator {
    pub fn new() -> Self {
        Accumulator { total: 0 }
    }

    pub fn add(&mut self, val: MyInt) {
        self.total += val as i64;
    }

    pub fn total(&self) -> MyInt {
        self.total as MyInt
    }
}

pub type MyAccumulator = Accumulator;

#[export_class]
impl Counter {
    fn get(&self) -> i64;
    fn increment(&mut self, delta: i64);
}

#[export_class]
impl Accumulator {
    fn add(&mut self, val: MyInt);
    fn total(&self) -> MyInt;
}
