use vector_basic::int_vec_new;

#[test]
fn vector_push_size_at() {
    let mut v = int_vec_new();
    v.push(10);
    v.push(20);
    v.push(30);
    assert_eq!(v.size(), 3);
    assert_eq!(v.at(0), 10);
    assert_eq!(v.at(1), 20);
    assert_eq!(v.at(2), 30);
}
