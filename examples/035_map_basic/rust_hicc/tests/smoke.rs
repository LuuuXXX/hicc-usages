use map_basic::{str_int_map_new, string_new};

#[test]
fn map_insert_lookup() {
    let mut m = str_int_map_new();
    let alice = string_new(b"alice\0".as_ptr() as *const i8);
    let bob   = string_new(b"bob\0".as_ptr() as *const i8);
    m.insert(&alice, 30);
    m.insert(&bob, 25);
    assert_eq!(m.size(), 2);
    assert!(m.contains(&alice));
    assert_eq!(m.get_or(&alice, -1), 30);
    let missing = string_new(b"zzz\0".as_ptr() as *const i8);
    assert_eq!(m.get_or(&missing, -1), -1);
}
