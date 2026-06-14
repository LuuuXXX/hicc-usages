// Virtual method bound as a regular method — the vtable dispatch is on the
// C++ side; Rust just calls the binding. std::string return requires the
// same import_class! treatment as in 001.

hicc::cpp! {
    #include "virtual_basic.h"

    inline std::string* hicc_string_new(const char* s) { return new std::string(s); }
    inline void         hicc_string_free(std::string* s) { delete s; }
}

hicc::import_class! {
    #[cpp(class = "std::string", destroy = "hicc_string_free")]
    pub class string {
        #[cpp(method = "const char* c_str() const")]
        pub fn c_str(&self) -> *const i8;
    }
}

hicc::import_class! {
    #[cpp(class = "Dog", destroy = "dog_free")]
    pub class Dog {
        // virtual — transparently bound:
        #[cpp(method = "std::string sound() const")]
        pub fn sound(&self) -> string;

        // inherited from Animal:
        #[cpp(method = "std::string name() const")]
        pub fn name(&self) -> string;
    }
}

hicc::import_lib! {
    #![link_name = "virtual_basic_hicc"]

    #[cpp(func = "Dog* dog_new(const char*)")]
    pub fn dog_new(name: *const i8) -> Dog;
}
