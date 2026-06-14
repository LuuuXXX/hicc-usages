#include "hicc_usages/class_constructor.h"
namespace hicc_usages::class_constructor {
Point::Point(int x, int y) : x_(x), y_(y) {}
Point* Point::create(int x, int y) { return new Point(x, y); }
void Point::free(Point* self) { delete self; }
int Point::get_x() const { return x_; }
int Point::get_y() const { return y_; }
int Point::distance_from_origin() const {
    return x_ * x_ + y_ * y_;
}
}
