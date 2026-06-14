use summary::*;
use std::ffi::CStr;

fn cs(s: &hicc_std::string) -> &str {
    unsafe { CStr::from_ptr(s.c_str()) }.to_str().unwrap_or("<bad utf-8>")
}

fn main() {
    let alice_name = hicc_std::string::from(c"alice");
    let mut c = Customer::new(1, &alice_name, CustomerTier::Basic);
    println!("c.describe() = {}", cs(&c.describe()));

    let _ = c.charge(50).ok();
    let _ = c.charge(100).ok();
    println!("c.total_spent() = {}", c.total_spent());
    println!("c.purchase_count() = {}", c.purchase_count());

    match c.charge(-5).ok() {
        Ok(_) => println!("charge ok"),
        Err(e) => println!("charge caught: {}", e.what()),
    }
    match c.purchase_at(99).ok() {
        Ok(v) => println!("purchase_at -> {}", v),
        Err(e) => println!("purchase_at caught: {}", e.what()),
    }

    let vip = VipCustomer::new(0.20);
    println!("vip.label() = {}", cs(&vip.label()));
    println!("vip.discount() = {}", vip.discount());
    println!("discounted price of 100 = {}", compute_discounted_price(&vip, 100.0));

    println!("MAX_CUSTOMERS = {}", *max_customers());
    println!("DEFAULT_DISCOUNT = {}", *default_discount());

    // vector 测试
    let mut v = vec_new();
    for x in [1, 3, 5, 7, 9] {
        v.push_back(&x);
    }
    println!("input size = {}", v.size());
    let doubled = doubled_values(&v);
    println!("doubled size = {}", doubled.size());
    println!("doubled slice = {:?}", doubled.as_slice());
}
