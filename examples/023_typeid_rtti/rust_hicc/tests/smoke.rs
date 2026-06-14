use std::ffi::CStr;
use typeid_rtti::{
    circle_new, static_type_name_circle, static_type_name_triangle, triangle_new,
    type_name_of_circle, type_name_of_triangle,
};

#[test]
fn rtti_dispatch_via_typeid() {
    let c = circle_new(2);
    let t = triangle_new(4, 6);

    let c_dyn = unsafe { CStr::from_ptr(type_name_of_circle(&c)).to_bytes().to_vec() };
    let t_dyn = unsafe { CStr::from_ptr(type_name_of_triangle(&t)).to_bytes().to_vec() };

    let c_static = unsafe { CStr::from_ptr(static_type_name_circle()).to_bytes().to_vec() };
    let t_static = unsafe { CStr::from_ptr(static_type_name_triangle()).to_bytes().to_vec() };

    // Dynamic type name from typeid() matches static type name.
    assert_eq!(c_dyn, c_static);
    assert_eq!(t_dyn, t_static);
    assert_ne!(c_dyn, t_dyn);

    assert_eq!(c.area(), 12);    // 3 * 2 * 2
    assert_eq!(t.area(), 12);    // 4 * 6 / 2
}
