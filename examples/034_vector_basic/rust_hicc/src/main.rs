use vector_basic::*;

fn main() {
    let mut v = vec_new();
    for x in [1, 3, 5, 7, 9] {
        v.push_back(&x);
    }
    println!("size = {}", v.size());
    println!("slice = {:?}", v.as_slice());
    println!("sum  = {}", vector_sum(&v));
    println!("avg  = {}", vector_avg(&v));
}
