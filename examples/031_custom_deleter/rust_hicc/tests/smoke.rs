use custom_deleter::{file_handle_fd, file_open};

#[test]
fn custom_deleter_wired() {
    let f = file_open(42);
    assert_eq!(file_handle_fd(&f), 42);
    // Drop fires file_close on scope exit (printed to stdout in this demo).
}
