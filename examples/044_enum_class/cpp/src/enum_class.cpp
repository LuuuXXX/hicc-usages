#include "hicc_usages/enum_class.h"
namespace hicc_usages::enum_class {

int color_to_int(Color c) { return static_cast<int>(c); }
Color int_to_color(int v) {
    if (v < 0 || v > 3) return Color::Red;
    return static_cast<Color>(v);
}
const char* color_name(Color c) {
    switch (c) {
        case Color::Red: return "Red";
        case Color::Green: return "Green";
        case Color::Blue: return "Blue";
        case Color::Yellow: return "Yellow";
    }
    return "Unknown";
}
bool is_primary(Color c) {
    return c == Color::Red || c == Color::Green || c == Color::Blue;
}

int direction_opposite(int d) {
    switch (static_cast<Direction>(d)) {
        case Direction::North: return static_cast<int>(Direction::South);
        case Direction::South: return static_cast<int>(Direction::North);
        case Direction::East: return static_cast<int>(Direction::West);
        case Direction::West: return static_cast<int>(Direction::East);
    }
    return -1;
}

Pixel::Pixel(int x, int y, Color c) : x_(x), y_(y), color_(c) {}
Pixel::~Pixel() = default;
Pixel* Pixel::create(int x, int y, Color c) { return new Pixel(x, y, c); }
void Pixel::free(Pixel* self) { delete self; }
int Pixel::x() const { return x_; }
int Pixel::y() const { return y_; }
Color Pixel::color() const { return color_; }
void Pixel::set_color(Color c) { color_ = c; }
bool Pixel::is_warm() const {
    return color_ == Color::Red || color_ == Color::Yellow;
}
}
