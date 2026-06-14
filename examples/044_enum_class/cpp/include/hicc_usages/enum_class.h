#pragma once
#include <cstddef>
#include <iostream>
namespace hicc_usages::enum_class {

enum class Color : int {
    Red = 0,
    Green = 1,
    Blue = 2,
    Yellow = 3,
};

enum class Direction : int {
    North = 0,
    South = 1,
    East = 2,
    West = 3,
};

int color_to_int(Color c);
Color int_to_color(int v);
const char* color_name(Color c);
bool is_primary(Color c);

int direction_opposite(int d);

class Pixel {
public:
    static Pixel* create(int x, int y, Color c);
    static void free(Pixel* self);
    int x() const;
    int y() const;
    Color color() const;
    void set_color(Color c);
    bool is_warm() const;
private:
    Pixel(int x, int y, Color c);
    ~Pixel();
    int x_;
    int y_;
    Color color_;
};

}  // namespace hicc_usages::enum_class
