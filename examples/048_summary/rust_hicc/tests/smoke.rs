use summary::*;
use std::ffi::CStr;

fn cs(s: &hicc_std::string) -> &str {
    unsafe { CStr::from_ptr(s.c_str()) }.to_str().unwrap_or("<bad utf-8>")
}

#[test]
fn customer_lifecycle() {
    let name = hicc_std::string::from(c"alice");
    let mut c = Customer::new(1, &name, CustomerTier::Basic);

    assert_eq!(c.id(), 1);
    assert_eq!(cs(&c.name()), "alice");
    assert_eq!(c.tier(), CustomerTier::Basic);
    assert_eq!(cs(&c.describe()), "Customer(1, alice, basic)");

    // charge + total（充值 + 累计）
    c.charge(50).ok().unwrap();
    c.charge(100).ok().unwrap();
    assert_eq!(c.purchase_count(), 2);
    assert_eq!(c.total_spent(), 150);

    // purchase_at 越界 -> Exception
    let err = c.purchase_at(99).ok().unwrap_err();
    assert!(err.what().contains("out of range"));
    assert_eq!(c.purchase_at(0).ok().unwrap(), 50);

    // charge 负值 -> Exception
    let err = c.charge(-5).ok().unwrap_err();
    assert!(err.what().contains("positive"));

    // 重命名
    let new_name = hicc_std::string::from(c"bob");
    c.rename(&new_name);
    assert_eq!(cs(&c.name()), "bob");
}

#[test]
fn customer_tier_upgrade() {
    let name = hicc_std::string::from(c"carol");
    let mut c = Customer::new(2, &name, CustomerTier::Basic);
    assert!(c.upgrade(CustomerTier::Premium as i32).ok().is_ok());
    assert_eq!(c.tier(), CustomerTier::Premium);

    // 降级应失败
    let err = c.upgrade(CustomerTier::Free as i32).ok().unwrap_err();
    assert!(err.what().contains("cannot downgrade"));
    assert_eq!(c.tier(), CustomerTier::Premium);
}

#[test]
fn vip_customer_and_inheritance() {
    let vip = VipCustomer::new(0.20);
    assert_eq!(cs(&vip.label()), "vip");
    assert!((vip.discount() - 0.20).abs() < 1e-10);
    assert!((compute_discounted_price(&vip, 100.0) - 80.0).abs() < 1e-6);
}

#[test]
fn vector_doubling() {
    let mut v = vec_new();
    for x in [1, 3, 5, 7, 9] {
        v.push_back(&x);
    }
    assert_eq!(v.size(), 5);
    let doubled = doubled_values(&v);
    assert_eq!(doubled.size(), 5);
    let slice = doubled.as_slice();
    assert_eq!(slice[0], 2);
    assert_eq!(slice[1], 6);
    assert_eq!(slice[2], 10);
    assert_eq!(slice[3], 14);
    assert_eq!(slice[4], 18);
}

#[test]
fn static_constexpr_data() {
    assert_eq!(*max_customers(), 1000);
    assert!((*default_discount() - 0.10).abs() < 1e-10);
}
