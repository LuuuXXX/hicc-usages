use mutable_member::*;

fn show(s: &hicc_std::string) -> String {
    let cs = unsafe { std::ffi::CStr::from_ptr(s.c_str()) };
    cs.to_str().unwrap().to_string()
}

fn main() {
    let key = hicc_std::string::from(c"user:1");
    let q = Query::new(&key);
    println!("exec1: {}", show(&q.execute()));
    println!("exec2: {}", show(&q.execute()));
    println!("call_count={}", q.call_count());
}
