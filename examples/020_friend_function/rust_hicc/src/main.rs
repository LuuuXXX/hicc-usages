use friend_function::*;

fn show(s: &hicc_std::string) -> String {
    let cs = unsafe { std::ffi::CStr::from_ptr(s.c_str()) };
    cs.to_str().unwrap().to_string()
}

fn main() {
    let o1 = hicc_std::string::from(c"Alice");
    let a = Account::new(&o1, 1000);
    let o2 = hicc_std::string::from(c"Bob");
    let b = Account::new(&o2, 2500);

    println!("{} ${}", show(&a.owner()), a.balance());
    println!("{} ${}", show(&b.owner()), b.balance());
    println!("audit_total(a)={}", audit_total(&a));
    println!("audit_total(b)={}", audit_total(&b));
}
