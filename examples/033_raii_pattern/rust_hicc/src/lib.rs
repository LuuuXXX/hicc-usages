// RAII: destroy= fires on Rust Drop → calls lock_free → ~Lock() releases.
// The C++ destructor is the RAII cleanup point; hicc maps it transparently.

hicc::cpp! {
    #include "raii_pattern.h"
}

hicc::import_class! {
    #[cpp(class = "Lock", destroy = "lock_free")]
    pub class Lock {
        #[cpp(method = "int id() const")]
        pub fn id(&self) -> i32;

        #[cpp(method = "bool is_locked() const")]
        pub fn is_locked(&self) -> bool;

        #[cpp(method = "void release()")]
        pub fn release(&mut self);
    }
}

hicc::import_lib! {
    #![link_name = "raii_pattern_hicc"]

    #[cpp(func = "Lock* lock_new(int)")]
    pub fn lock_new(id: i32) -> Lock;
}
