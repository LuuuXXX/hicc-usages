use noexcept_basic::*;
use std::ffi::CStr;

fn cs(s: &hicc_std::string) -> &str {
    unsafe { CStr::from_ptr(s.c_str()) }.to_str().unwrap_or("<bad utf-8>")
}

fn main() {
    println!("add_noexcept(2, 3) = {}", add_noexcept(2, 3));
    println!("square_noexcept(7) = {}", square_noexcept(7));
    println!("safe_reciprocal(4) = {}", safe_reciprocal_noexcept(4.0));
    println!("safe_reciprocal(0) = {}", safe_reciprocal_noexcept(0.0));
    println!("compute_constant() = {}", compute_constant());

    let mut counter = SafeCounter::new();
    counter.increment(5);
    counter.increment(3);
    println!("counter.get() = {}", counter.get());
    println!("counter.describe() = {}", cs(&counter.describe()));

    let mut buf = Buffer::new(4);
    buf.set(0, 10);
    buf.set(3, 40);
    println!("buf.size() = {}", buf.size());
    println!("buf.get(0) = {}", buf.get(0));
    println!("buf.get(3) = {}", buf.get(3));

    // may_throw — use Exception<T> path for safety
    match may_throw(5).ok() {
        Ok(v) => println!("may_throw(5) = {}", v),
        Err(e) => println!("caught: {}", e.what()),
    }
    match may_throw(-1).ok() {
        Ok(v) => println!("may_throw(-1) = {}", v),
        Err(e) => println!("caught: {}", e.what()),
    }
}
