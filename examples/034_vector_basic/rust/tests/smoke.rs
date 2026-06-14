//! 自动生成：hicc_usage_vector_basic 冒烟测试
//!
//! 简单穿透调用：验证 FFI 链路（cpp → libxxx.a → hicc adapter → rust）通畅。
//! 不覆盖全部 API，只验证可编译可链接可调用。

#[test]
fn smoke_links_and_calls() {
    // 调用 factory 创建实例（不 panic 即通过）
    let _obj = hicc_usage_vector_basic::intvector_new();
    let _ = ();
}
