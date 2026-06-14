use friend_function::*;

fn show(s: &hicc_std::string) -> String {
    let cs = unsafe { std::ffi::CStr::from_ptr(s.c_str()) };
    cs.to_str().unwrap().to_string()
}

#[test]
fn friend_audit_total() {
    let o1 = hicc_std::string::from(c"Alice");
    let a = Account::new(&o1, 1000);
    let o2 = hicc_std::string::from(c"Bob");
    let b = Account::new(&o2, 2500);

    assert_eq!(show(&a.owner()), "Alice");
    assert_eq!(a.balance(), 1000);
    assert_eq!(audit_total(&a), 1000);
    assert_eq!(audit_total(&b), 2500);
}
