use std::ffi::CString;
use variadic_functions::*;

fn main() {
    // hicc 约定：变长参数函数被生成为「无参 → 函数指针」形式
    // 调用：sum_ints()(count, args...)
    let total = unsafe { sum_ints()(3, 10, 20, 30) };
    println!("sum_ints(3, 10, 20, 30) = {}", total);

    let fmt = CString::new("rust log: %s=%d\n").unwrap();
    let key = CString::new("answer").unwrap();
    unsafe { log_line()(fmt.as_ptr(), key.as_ptr(), 42) };
}
