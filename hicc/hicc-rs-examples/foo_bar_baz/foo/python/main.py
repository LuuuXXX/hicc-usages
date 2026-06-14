"""Python test for foo_bar_baz cross-crate example."""
from foo_bar_baz import (add_points, make_point, make_line,
                          rect_area, make_rect,
                          new_counter, counter_inc, counter_get,
                          new_accumulator, accumulator_add, accumulator_total,
                          Point, Line, Rectangle, Counter, Accumulator)


def main():
    # 1. Point POD (from bar crate, accessed via MyPoint type alias)
    p1 = Point(10, 20)
    p2 = Point(1, 2)
    sp = add_points(p1, p2)
    print(f"add_points((10,20), (1,2)) = ({sp.x}, {sp.y})")

    p3 = make_point(5, 15)
    print(f"make_point(5,15) = ({p3.x}, {p3.y})")

    # 2. Rectangle POD (from baz crate, accessed via MyRect type alias)
    r = Rectangle(3.5, 2.0)
    a = rect_area(r)
    print(f"rect_area({3.5, 2.0}) = {a}")

    r2 = make_rect(4.0, 5.0)
    a2 = rect_area(r2)
    print(f"rect_area(make_rect(4,5)) = {a2}")

    # 3. Line POD using MyPoint (type alias) fields
    line = make_line(Point(3, 7), Point(10, 20))
    print(f"make_line((3,7), (10,20)) = start({line.start.x}, {line.start.y}), end({line.end.x}, {line.end.y})")

    # 4. Counter (export_class from bar crate, accessed via MyCounter alias)
    c = new_counter(100)
    val = counter_get(c)
    print(f"counter_get(new_counter(100)) = {val}")

    inc = counter_inc(c, 50)
    print(f"counter_inc(c, 50) = {inc}")

    c.destroy()

    # 5. Accumulator (export_class using MyInt = i32 type alias)
    a = new_accumulator()
    r1 = accumulator_add(a, 10)
    print(f"accumulator_add(a, 10) = {r1}")
    r2 = accumulator_add(a, 20)
    print(f"accumulator_add(a, 20) = {r2}")
    tot = accumulator_total(a)
    print(f"accumulator_total(a) = {tot}")
    a.destroy()

    print("Cross-crate foo_bar_baz example passed!")


if __name__ == "__main__":
    main()
