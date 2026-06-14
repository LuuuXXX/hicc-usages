use inheritance_single::*;

fn show(s: &hicc_std::string) -> String {
    let cs = unsafe { std::ffi::CStr::from_ptr(s.c_str()) };
    cs.to_str().unwrap().to_string()
}

#[test]
fn dog_polymorphism() {
    let n = hicc_std::string::from(c"Rex");
    let d = Dog::new(&n);
    assert_eq!(show(&d.name()), "Rex");
    assert_eq!(show(&d.sound()), "Woof");
    assert_eq!(d.legs(), 4);
    assert_eq!(show(&d.breed()), "Unknown");
}

#[test]
fn cat_polymorphism() {
    let n = hicc_std::string::from(c"Mimi");
    let c = Cat::new(&n);
    assert_eq!(show(&c.name()), "Mimi");
    assert_eq!(show(&c.sound()), "Meow");
    assert_eq!(c.legs(), 4);
}
