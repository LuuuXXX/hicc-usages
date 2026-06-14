#pragma once

// explicit constructor: prevents implicit conversion. From Rust's perspective
// there's no difference — still bound through a factory function.

class Celsius {
public:
    explicit Celsius(double v) : value_(v) {}
    double value() const { return value_; }
private:
    double value_;
};

class Fahrenheit {
public:
    Celsius to_celsius() const;
    explicit Fahrenheit(double v) : value_(v) {}
    double value() const { return value_; }
private:
    double value_;
};

Celsius*   celsius_new(double v);
Fahrenheit* fahrenheit_new(double v);
void       celsius_free(Celsius*);
void       fahrenheit_free(Fahrenheit*);
Celsius    convert_to_celsius(const Fahrenheit& f);  // wrapper for explicit conv
