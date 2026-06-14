use exception_basic::*;

fn main() {
    // 1. safe_divide success
    match safe_divide(10, 3).ok() {
        Ok(v) => println!("10/3 = {}", v),
        Err(e) => println!("caught: {}", e.what()),
    }
    // 2. safe_divide error
    match safe_divide(10, 0).ok() {
        Ok(v) => println!("10/0 = {}", v),
        Err(e) => println!("caught: {}", e.what()),
    }
    // 3. BankAccount
    let mut acc = BankAccount::new(100);
    println!("balance = {}", acc.balance());
    let _ = acc.deposit(50).ok();
    println!("after deposit balance = {}", acc.balance());
    match acc.withdraw(1000).ok() {
        Ok(v) => println!("withdrew {}", v),
        Err(e) => println!("withdraw caught: {}", e.what()),
    }
    match acc.deposit(-5).ok() {
        Ok(_) => println!("deposit ok"),
        Err(e) => println!("deposit caught: {}", e.what()),
    }
    // 4. parse_int
    match parse_int(&hicc_std::string::from(c"42")).ok() {
        Ok(v) => println!("parsed 42 -> {}", v),
        Err(e) => println!("parse_int caught: {}", e.what()),
    }
    match parse_int(&hicc_std::string::from(c"not_a_number")).ok() {
        Ok(v) => println!("parsed -> {}", v),
        Err(e) => println!("parse_int caught: {}", e.what()),
    }
}
