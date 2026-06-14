use exception_basic::*;

#[test]
fn safe_divide_ok_and_err() {
    assert_eq!(safe_divide(10, 3).ok().unwrap(), 3);
    let err = safe_divide(10, 0).ok().unwrap_err();
    assert!(err.what().contains("division by zero"));
}

#[test]
fn bank_account_deposit_and_overdraw() {
    let mut acc = BankAccount::new(100);
    assert_eq!(acc.balance(), 100);
    acc.deposit(50).ok().unwrap();
    assert_eq!(acc.balance(), 150);

    // 取款过多 -> 错误
    let err = acc.withdraw(1000).ok().unwrap_err();
    assert!(err.what().contains("insufficient funds"));
    // 余额不应改变
    assert_eq!(acc.balance(), 150);

    // 合法取款
    let got = acc.withdraw(50).ok().unwrap();
    assert_eq!(got, 50);
    assert_eq!(acc.balance(), 100);

    // 存入负值
    let err = acc.deposit(-5).ok().unwrap_err();
    assert!(err.what().contains("non-negative"));
}

#[test]
fn parse_int_ok_and_err() {
    assert_eq!(parse_int(&hicc_std::string::from(c"42")).ok().unwrap(), 42);
    let err = parse_int(&hicc_std::string::from(c"not_a_number")).ok().unwrap_err();
    assert!(err.what().contains("invalid integer"));
}
