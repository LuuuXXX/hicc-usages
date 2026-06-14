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

#[test]
fn friend_accesses_private_field() {
    let owner = hicc_std::string::from(c"Carol");
    let a = Account::new(&owner, 500);
    let b = Account::new(&owner, 750);
    // friend function 能读 private balance_ 字段，跨多个实例求和
    assert_eq!(audit_total(&a) + audit_total(&b), 1250);
}
