#![feature(specialization)]

fn main() {
    let mut v: Vec<String> = Vec::new();
    v.push("hello".to_string());
    assert_eq!(v.len(), 1);
    assert_eq!(v[0], "hello");
    println!("native_vec_push: OK");

    let s = "hello".to_string();
    assert_eq!(s.len(), 5);
    assert_eq!(s.as_str(), "hello");
    println!("native_string: OK");

    println!("Foreign type example passed!");
}
