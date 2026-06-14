//! 自动生成：hicc_usage_template_function 冒烟测试
//!
//! 简单穿透调用：验证 FFI 链路（cpp → libxxx.a → hicc adapter → rust）通畅。
//! 不覆盖全部 API，只验证可编译可链接可调用。

#[test]
fn smoke_links_and_calls() {
    // 调用一个模板实例化函数（不 panic 即通过）
    hicc_usage_template_function::max_of_int(0, 0);
    let _ = ();
}
