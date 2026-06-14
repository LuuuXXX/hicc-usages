// explicit ctor: doesn't affect FFI binding — still exposed via factory.
// Conversion member functions like `Fahrenheit::to_celsius` are wrapped
// because they return by value (Celsius is owned by Rust after the call).

hicc::cpp! {
    #include "explicit_ctor.h"
}

hicc::import_class! {
    #[cpp(class = "Celsius", destroy = "celsius_free")]
    pub class Celsius {
        #[cpp(method = "double value() const")]
        pub fn value(&self) -> f64;
    }
}

hicc::import_class! {
    #[cpp(class = "Fahrenheit", destroy = "fahrenheit_free")]
    pub class Fahrenheit {
        #[cpp(method = "double value() const")]
        pub fn value(&self) -> f64;
    }
}

hicc::import_lib! {
    #![link_name = "explicit_ctor_hicc"]

    #[cpp(func = "Celsius* celsius_new(double)")]
    pub fn celsius_new(v: f64) -> Celsius;

    #[cpp(func = "Fahrenheit* fahrenheit_new(double)")]
    pub fn fahrenheit_new(v: f64) -> Fahrenheit;

    #[cpp(func = "Celsius convert_to_celsius(const Fahrenheit&)")]
    pub fn convert_to_celsius(f: &Fahrenheit) -> Celsius;
}
