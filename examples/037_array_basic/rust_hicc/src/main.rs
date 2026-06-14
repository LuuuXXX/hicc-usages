use array_basic::*;

fn main() {
    let mut a = array5_new();
    fill_array(&mut a, 10);  // [10,11,12,13,14]

    println!("size = {}", a.size());
    println!("slice = {:?}", a.as_slice());
    println!("sum   = {}", array_sum(&a));
    println!("max   = {}", array_max(&a));
    println!("avg   = {}", array_avg(&a));
}
