use unique_ptr::make_widget;

#[test]
fn unique_ptr_unwrapped_to_owned() {
    let w = make_widget(42);
    assert_eq!(w.value(), 42);
    // Drop fires on scope exit, calling widget_free.
}
