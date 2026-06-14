use typeid_rtti::*;

fn show(p: *const i8) -> String {
    if p.is_null() { return "<null>".to_string(); }
    let cs = unsafe { std::ffi::CStr::from_ptr(p) };
    cs.to_str().unwrap().to_string()
}

fn main() {
    let a = DerivedA::new();
    let b = DerivedB::new();
    println!("a.name={}", show_str(&a.name()));
    println!("b.name={}", show_str(&b.name()));
    println!("a.typeid.name={}", show(type_name_base_a(&a)));
    println!("same_type(a,a)={}", same_type_a_a(&a, &a));
    println!("same_type(a,b)={}", same_type_a_b(&a, &b));
    println!("is_derived_a(a)={}", is_derived_a_a(&a));
    println!("is_derived_a(b)={}", is_derived_a_b(&b));
}

fn show_str(s: &hicc_std::string) -> String {
    let cs = unsafe { std::ffi::CStr::from_ptr(s.c_str()) };
    cs.to_str().unwrap().to_string()
}
