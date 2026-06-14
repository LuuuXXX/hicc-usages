use vector_basic::*;

#[test]
fn vector_push_sum_avg() {
    let mut v = vec_new();
    assert!(v.is_empty());
    for x in [1, 3, 5, 7, 9] {
        v.push_back(&x);
    }
    assert_eq!(v.size(), 5);
    assert_eq!(v.as_slice(), &[1, 3, 5, 7, 9]);
    assert_eq!(vector_sum(&v), 25);
    assert_eq!(vector_avg(&v), 5.0);
}

#[test]
fn vector_back_front_accessors() {
    let mut v = vec_new();
    v.push_back(&10);
    v.push_back(&20);
    v.push_back(&30);
    assert_eq!(*v.front().unwrap(), 10);
    assert_eq!(*v.back().unwrap(), 30);
    // mutable back
    *v.back_mut().unwrap() += 1;
    assert_eq!(*v.back().unwrap(), 31);
}
