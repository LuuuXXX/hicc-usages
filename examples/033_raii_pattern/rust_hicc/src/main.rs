use raii_pattern::*;

fn show(s: &hicc_std::string) -> String {
    let cs = unsafe { std::ffi::CStr::from_ptr(s.c_str()) };
    cs.to_str().unwrap().to_string()
}

fn main() {
    let path = hicc_std::string::from(c"/tmp/raii_demo.txt");
    let mut f = FileHandle::open(7, &path);
    println!("opened fd={} path={}", f.fd(), show(&f.path()));

    let d1 = hicc_std::string::from(c"hello");
    let d2 = hicc_std::string::from(c" world");
    let n1 = f.write(&d1);
    let n2 = f.write(&d2);
    println!("wrote {} + {} bytes", n1, n2);
    println!("size={}", f.size());
    println!("avail={}", read_file(&mut f));

    let fd = f.fd();
    drop(f); // RAII：释放时触发 C++ 析构
    println!("dropped fd={}", fd);
}
