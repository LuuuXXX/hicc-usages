use virtual_pure::*;

fn show(s: &hicc_std::string) -> String {
    let cs = unsafe { std::ffi::CStr::from_ptr(s.c_str()) };
    cs.to_str().unwrap().to_string()
}

fn main() {
    let mut s = InMemoryStorage::new();
    let k1 = hicc_std::string::from(c"name");
    let v1 = hicc_std::string::from(c"Alice");
    s.put(&k1, &v1);

    let k2 = hicc_std::string::from(c"age");
    let v2 = hicc_std::string::from(c"30");
    s.put(&k2, &v2);

    println!("size={}", s.size());
    println!("name={}", show(&s.get(&k1)));

    s.dump();
}
