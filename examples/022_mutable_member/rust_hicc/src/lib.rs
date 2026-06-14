// mutable member is transparent — `compute()` is const from C++ perspective
// (even though it mutates mutable internals), so Rust sees `&self`.

hicc::cpp! {
    #include "mutable_member.h"
}

hicc::import_class! {
    #[cpp(class = "Cache", destroy = "cache_free")]
    pub class Cache {
        #[cpp(method = "int compute(int) const")]
        pub fn compute(&self, x: i32) -> i32;

        #[cpp(method = "int last_cached() const")]
        pub fn last_cached(&self) -> i32;
    }
}

hicc::import_lib! {
    #![link_name = "mutable_member_hicc"]

    #[cpp(func = "Cache* cache_new()")]
    pub fn cache_new() -> Cache;
}
