// Custom deleter: the destroy= attribute points to a free function that
// performs the cleanup (file_close, not delete).

hicc::cpp! {
    #include "custom_deleter.h"
}

hicc::import_class! {
    // destroy= calls file_close, which does user-defined cleanup (close + delete).
    #[cpp(class = "FileHandle", destroy = "file_close")]
    pub class FileHandle {
        // public field — accessed via a getter since hicc methods are needed
    }
}

hicc::cpp! {
    inline int file_handle_fd(const FileHandle* h) { return h->fd; }
}

hicc::import_lib! {
    #![link_name = "custom_deleter_hicc"]

    // Factory using FileHandle* (not unique_ptr) so the destroy= path is clean.
    #[cpp(func = "FileHandle* file_open(int)")]
    pub fn file_open(fd: i32) -> FileHandle;

    // Accessor for the fd field.
    #[cpp(func = "int file_handle_fd(const FileHandle*)")]
    pub fn file_handle_fd(h: &FileHandle) -> i32;
}
