use class_volatile::*;

fn main() {
    let mut s = Sensor::new(42);
    s.safe_write(100);
    println!("id={} read={} counter={}", s.id(), s.safe_read(), s.counter());
    s.safe_write(200);
    println!("after second write read={} counter={}", s.safe_read(), s.counter());
}
