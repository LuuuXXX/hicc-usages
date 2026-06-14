use tuple_basic::*;

fn show(s: &hicc_std::string) -> String {
    let cs = unsafe { std::ffi::CStr::from_ptr(s.c_str()) };
    cs.to_str().unwrap().to_string()
}

fn main() {
    let t = make_triple(7, &hicc_std::string::from(c"alice"), 9.5);
    println!("id    = {}", triple_id(&t));
    println!("name  = {}", show(&triple_name(&t)));
    println!("score = {}", triple_score(&t));

    let mut t2 = make_triple(1, &hicc_std::string::from(c"bob"), 7.0);
    set_id(&mut t2, 99);
    set_score(&mut t2, 8.8);
    println!("after update id={} score={}", triple_id(&t2), triple_score(&t2));
}
