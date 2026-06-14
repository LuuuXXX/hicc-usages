use mutable_member::cache_new;

#[test]
fn mutable_cache_via_const_method() {
    let c = cache_new();
    assert_eq!(c.compute(5), 25);
    assert_eq!(c.compute(5), 25);  // second call uses cache
    assert_eq!(c.last_cached(), 25);
}
