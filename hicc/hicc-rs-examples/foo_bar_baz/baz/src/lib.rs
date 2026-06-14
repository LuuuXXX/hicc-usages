#![feature(specialization)]

use hicc_rs::*;

use std::fmt::Write;

// ---- Basic type alias ----
pub type MyDouble = f64;

// ---- POD struct ----
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

// ---- POD type alias ----
pub type MyRect = Rectangle;

// ---- Non-POD struct ----
pub struct Greeter {
    greeting: String,
}

impl Greeter {
    pub fn new(greeting: String) -> Self {
        Greeter { greeting }
    }

    pub fn greet(&self, name: &str) -> String {
        let mut out = String::new();
        let _ = write!(&mut out, "{} {}", self.greeting, name);
        out
    }

    pub fn set_greeting(&mut self, greeting: String) {
        self.greeting = greeting;
    }
}

// ---- Non-POD type alias ----
pub type MyGreeter = Greeter;

#[export_class]
impl Greeter {
    fn greet(&self, name: &str) -> String;
    fn set_greeting(&mut self, greeting: String);
}
