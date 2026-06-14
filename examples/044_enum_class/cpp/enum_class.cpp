#include "enum_class.h"

namespace enum_class_ns {

std::string color_name(Color c) {
    switch (c) {
        case Color::Red:   return "red";
        case Color::Green: return "green";
        case Color::Blue:  return "blue";
    }
    return "unknown";
}

Color color_parse(const std::string& s) {
    if (s == "red")   return Color::Red;
    if (s == "green") return Color::Green;
    if (s == "blue")  return Color::Blue;
    return Color::Red;
}

Light::Light(Color initial) : color_(initial) {}
Color Light::current() const { return color_; }
void Light::set(Color c) { color_ = c; }
int Light::brightness() const {
    switch (color_) {
        case Color::Red:   return 100;
        case Color::Green: return 200;
        case Color::Blue:  return 300;
    }
    return 0;
}

Light make_light(Color initial) { return Light(initial); }

int enum_class_anchor() { return 44; }

} // namespace enum_class_ns
