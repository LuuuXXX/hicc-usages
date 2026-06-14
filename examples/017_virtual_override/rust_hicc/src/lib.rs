// Override is transparent at the FFI layer — each derived class is a separate
// Rust type, with its own destroy function and its own (overridden) methods.

hicc::cpp! {
    #include "virtual_override.h"

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
    #[cpp(class = "InfoLogger", destroy = "logger_free_info")]
    pub class InfoLogger {
        #[cpp(method = "std::string level() const")]
        pub fn level(&self) -> string;

        #[cpp(method = "std::string format(const std::string&) const")]
        pub fn format(&self, msg: &string) -> string;
    }
}

hicc::import_class! {
    #[cpp(class = "ErrorLogger", destroy = "logger_free_error")]
    pub class ErrorLogger {
        #[cpp(method = "std::string level() const")]
        pub fn level(&self) -> string;

        #[cpp(method = "std::string format(const std::string&) const")]
        pub fn format(&self, msg: &string) -> string;
    }
}

hicc::import_lib! {
    #![link_name = "virtual_override_hicc"]

    #[cpp(func = "InfoLogger* info_logger_new()")]
    pub fn info_logger_new() -> InfoLogger;

    #[cpp(func = "ErrorLogger* error_logger_new()")]
    pub fn error_logger_new() -> ErrorLogger;

    #[cpp(func = "std::string* hicc_string_new(const char*)")]
    pub fn string_new(c_str: *const i8) -> string;
}
