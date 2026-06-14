use friend_function::{account_new, merge};

#[test]
fn friend_function_bindable() {
    let a = account_new(100);
    let b = account_new(50);
    let c = merge(&a, &b);
    assert_eq!(c.balance(), 150);
}
