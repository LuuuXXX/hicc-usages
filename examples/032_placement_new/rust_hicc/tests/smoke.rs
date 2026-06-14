use placement_new::{Buffer, Placement};

#[test]
fn buffer_constructs() {
    let buf = Buffer::new(64);
    assert_eq!(buf.size(), 64);
}

#[test]
fn placement_new_raw_basic() {
    let mut mem = vec![0u8; 64];
    let p = Placement::new(&mut mem, 7);
    assert_eq!(p.value(), 7);
}

#[test]
fn placement_set_get() {
    let mut mem = vec![0u8; 64];
    let mut p = Placement::new(&mut mem, 1);
    assert_eq!(p.value(), 1);
    p.set(99);
    assert_eq!(p.value(), 99);
}
