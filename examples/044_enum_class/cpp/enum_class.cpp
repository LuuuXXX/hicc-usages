#include "enum_class.h"

int color_to_int(Color c) { return static_cast<int>(c); }
Color int_to_color(int v) {
    switch (v) {
        case 0: return Color::Red;
        case 1: return Color::Green;
        case 2: return Color::Blue;
        default: return Color::Red;
    }
}
const char* color_name(Color c) {
    switch (c) {
        case Color::Red: return "red";
        case Color::Green: return "green";
        case Color::Blue: return "blue";
    }
    return "?";
}

int to_int_red() { return static_cast<int>(Color::Red); }
int to_int_green() { return static_cast<int>(Color::Green); }
int to_int_blue() { return static_cast<int>(Color::Blue); }
const char* color_name_for_int(int v) { return color_name(int_to_color(v)); }
