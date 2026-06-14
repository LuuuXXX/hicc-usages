#pragma once
namespace hicc_usages::inheritance_multiple {
class Drawable {
public:
    static int draw_calls();
    const char* shape_name() const;
protected:
    static int draw_calls_;
};
class Printable {
public:
    static int print_calls();
    const char* printable_text() const;
protected:
    static int print_calls_;
};
class Shape : public Drawable, public Printable {
public:
    static Shape* create();
    static void free(Shape* self);
    const char* shape_name() const;
    const char* printable_text() const;
};
}
