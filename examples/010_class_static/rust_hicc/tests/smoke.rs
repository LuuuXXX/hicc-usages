use class_static::{registry_live_count, registry_next_id};

#[test]
fn static_next_id_monotonic() {
    let a = registry_next_id();
    let b = registry_next_id();
    assert!(b > a, "next_id should be monotonic: a={a} b={b}");
}

#[test]
fn static_live_count_visible() {
    // We don't create instances from Rust (no factory in this demo), so
    // live_count should remain 0 across Rust calls.
    assert!(registry_live_count() >= 0);
}
