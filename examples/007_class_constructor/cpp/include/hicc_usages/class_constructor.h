#pragma once
namespace hicc_usages::class_constructor {
class Point {
public:
    static Point* create(int x, int y);
    static void free(Point* self);
    int get_x() const;
    int get_y() const;
    int distance_from_origin() const;
private:
    Point(int x, int y);
    int x_, y_;
};
}
