use constexpr_basic::*;

fn main() {
    println!("PI = {}", *pi());
    println!("E = {}", *e_constant());
    println!("BUFFER_SIZE = {}", *buffer_size());
    println!("MAX_TRIES = {}", *max_tries());
    println!("BIG_NUMBER = {}", *big_number());

    println!("square(7) = {}", square(7));
    println!("factorial(5) = {}", factorial(5));

    println!("compute_area(2.0) = {}", compute_area(2.0));
}
