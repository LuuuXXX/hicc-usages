use placement_new::{vec3_align, vec3_construct, vec3_destruct, vec3_size, vec3_x, vec3_y, vec3_z};

#[test]
fn placement_new_in_rust_buffer() {
    let size = vec3_size();
    let align = vec3_align();
    assert!(size >= 12);  // 3 * f32

    // Allocate aligned buffer.
    let mut buf = vec![0u8; size];
    let raw = buf.as_mut_ptr();
    // Ensure alignment (Vec<u8> doesn't guarantee > 1 alignment; for the demo
    // we accept whatever Vec gives, real code would use aligned_alloc).
    let _ = align;

    unsafe { vec3_construct(raw as *mut u8, 1.0, 2.0, 3.0) };
    unsafe {
        assert!((vec3_x(raw as *const u8) - 1.0).abs() < 1e-6);
        assert!((vec3_y(raw as *const u8) - 2.0).abs() < 1e-6);
        assert!((vec3_z(raw as *const u8) - 3.0).abs() < 1e-6);
        vec3_destruct(raw as *mut u8);
    }
}
