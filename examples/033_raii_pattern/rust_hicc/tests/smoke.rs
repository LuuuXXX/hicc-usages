use raii_pattern::*;

fn show(s: &hicc_std::string) -> String {
    let cs = unsafe { std::ffi::CStr::from_ptr(s.c_str()) };
    cs.to_str().unwrap().to_string()
}

#[test]
fn raii_factory_and_drop() {
    let path = hicc_std::string::from(c"/tmp/x.txt");
    let mut f = FileHandle::open(11, &path);
    assert_eq!(f.fd(), 11);
    assert_eq!(show(&f.path()), "/tmp/x.txt");

    let d = hicc_std::string::from(c"data");
    let n = f.write(&d);
    assert_eq!(n, 4);
    assert_eq!(f.size(), 4);

    // RAII Drop == C++ dtor: simply verify final state before drop.
    let fd = f.fd();
    drop(f);
    assert_eq!(fd, 11);
}

#[test]
fn raii_read_file_helper() {
    let path = hicc_std::string::from(c"/tmp/y.txt");
    let mut f = FileHandle::open(22, &path);
    let d = hicc_std::string::from(c"abc");
    let _ = f.write(&d);
    assert_eq!(read_file(&mut f), 3);
}
