#pragma once

// Parameterized constructor — exposed to Rust via a factory function
// (hicc convention: constructors are not bound directly, only via factory).

class Point {
public:
    Point(int x, int y) : x_(x), y_(y) {}
    int get_x() const { return x_; }
    int get_y() const { return y_; }
    int  manhattan() const { return (x_ < 0 ? -x_ : x_) + (y_ < 0 ? -y_ : y_); }
private:
    int x_, y_;
};

Point* point_new(int x, int y);
void   point_free(Point* p);
