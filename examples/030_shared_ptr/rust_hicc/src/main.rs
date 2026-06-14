use shared_ptr::*;

fn main() {
    let c1 = make_counter(10);
    println!("c1 value={} use_count={}", c1.get().value(), use_count(&c1));

    let c2 = clone_counter(&c1);
    println!("after clone use_count={}", use_count(&c1));

    c2.get().increment();
    println!("after c2.increment c1 value={}", c1.get().value());
}
