use default_args::*;

fn main() {
    let name = hicc_std::string::from(c"Rust");
    let bang = hicc_std::string::from(c"!");
    let q = hicc_std::string::from(c"?");

    println!("greet_default = {}", greet_default(&name));
    println!("greet_times(2) = {}", greet_times(&name, 2));
    println!("greet_full(3, ?) = {}", greet_full(&name, 3, &q));

    println!("compute_one(1) = {}", compute_one(1));
    println!("compute_two(1, 2) = {}", compute_two(1, 2));
    println!("compute_full(1, 2, 3) = {}", compute_full(1, 2, 3));

    let _ = (name, bang, q);
}
