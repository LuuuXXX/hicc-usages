use std::ffi::CStr;
use virtual_basic::dog_new;

#[test]
fn virtual_dispatch_through_binding() {
    let d = dog_new(b"Rex\0".as_ptr() as *const i8);
    let sound = d.sound();
    let name = d.name();
    unsafe {
        assert_eq!(CStr::from_ptr(sound.c_str()).to_bytes(), b"Woof!");
        assert_eq!(CStr::from_ptr(name.c_str()).to_bytes(), b"Rex");
    }
}
