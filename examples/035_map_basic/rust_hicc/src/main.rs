use map_basic::*;

fn show(s: &hicc_std::string) -> String {
    let cs = unsafe { std::ffi::CStr::from_ptr(s.c_str()) };
    cs.to_str().unwrap().to_string()
}

fn main() {
    let mut m = map_new();
    put(&mut m, 1, &hicc_std::string::from(c"one"));
    put(&mut m, 2, &hicc_std::string::from(c"two"));
    put(&mut m, 3, &hicc_std::string::from(c"three"));

    println!("size = {}", map_size(&m));
    println!("key2 = {}", show(&get_or(&m, 2, &hicc_std::string::from(c"?"))));
    println!("key5 = {}", show(&get_or(&m, 5, &hicc_std::string::from(c"missing"))));
    println!("sum_keys = {}", sum_key_values(&m));

    // hicc_std::map 内置 insert/get
    let n4 = hicc_std::string::from(c"four");
    m.insert(&4, &n4);
    if let Some(v) = m.get(&4) {
        println!("key4 (native) = {}", show(&v));
    }
}
