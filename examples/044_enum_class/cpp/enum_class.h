#pragma once
#include <string>
#include <iostream>

namespace enum_class_ns {

// Primary enum class
enum class Color { Red, Green, Blue };

// Second enum class with explicit values
enum class Status : int { Active = 10, Inactive = 20, Pending = 30 };

// Wrappers to convert enum<->int (hicc can't FFI enum class directly)
inline int color_to_int(Color c) { return static_cast<int>(c); }
inline Color color_from_int(int v) {
    if (v < 0 || v > 2) return Color::Red;  // safe default
    return static_cast<Color>(v);
}
inline int status_to_int(Status s) { return static_cast<int>(s); }
inline Status status_from_int(int v) {
    if (v == 10) return Status::Active;
    if (v == 20) return Status::Inactive;
    if (v == 30) return Status::Pending;
    return Status::Pending;
}

// String helpers
std::string color_name(Color c);
Color color_parse(const std::string& s);

// Class that takes/returns enum
class Light {
public:
    Light(Color initial);
    Color current() const;
    void set(Color c);
    int brightness() const;       // returns an int derived from Color (Red=100, Green=200, Blue=300)
private:
    Color color_;
};

Light make_light(Color initial);

} // namespace enum_class_ns
