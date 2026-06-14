#include <stdio.h>
#include <inttypes.h>
#include "foo_bar_baz.h"

int main(void) {
    const struct Hicc_foo_bar_baz *fn = foo_bar_baz();

    /* 1. Point POD (from bar crate) */
    {
        struct Point p1 = {10, 20};
        struct Point p2 = {1, 2};
        struct Point sum = fn->add_points(p1, p2);
        printf("add_points((10,20), (1,2)) = (%" PRId32 ", %" PRId32 ")\n", sum.x, sum.y);

        struct Point p3 = fn->make_point(5, 15);
        printf("make_point(5,15) = (%" PRId32 ", %" PRId32 ")\n", p3.x, p3.y);
    }

    /* 2. Rectangle POD (from baz crate) */
    {
        struct Rectangle r = {3.5, 2.0};
        double a = fn->rect_area(r);
        printf("rect_area({3.5, 2.0}) = %.1f\n", a);

        struct Rectangle r2 = fn->make_rect(4.0, 5.0);
        double a2 = fn->rect_area(r2);
        printf("rect_area(make_rect(4,5)) = %.1f\n", a2);
    }

    /* 3. Line POD (using MyPoint = Point type alias for fields) */
    {
        struct Point p1 = {3, 7};
        struct Point p2 = {10, 20};
        struct Line line = fn->make_line(p1, p2);
        printf("make_line((3,7), (10,20)) = start(%" PRId32 ", %" PRId32 "), end(%" PRId32 ", %" PRId32 ")\n",
               line.start.x, line.start.y, line.end.x, line.end.y);
    }

    /* 4. Counter (export_class from bar crate) */
    {
        struct AbiClass_Counter c = fn->new_counter(100);
        int64_t val = fn->counter_get(&c);
        printf("counter_get(new_counter(100)) = %" PRId64 "\n", val);

        int64_t inc = fn->counter_inc(&c, 50);
        printf("counter_inc(c, 50) = %" PRId64 "\n", inc);

        c.methods->hicc_destroy(c);
    }

    /* 5. Accumulator (export_class using MyInt = i32 type alias) */
    {
        struct AbiClass_Accumulator a = fn->new_accumulator();
        int32_t r1 = fn->accumulator_add(&a, 10);
        printf("accumulator_add(a, 10) = %" PRId32 "\n", r1);
        int32_t r2 = fn->accumulator_add(&a, 20);
        printf("accumulator_add(a, 20) = %" PRId32 "\n", r2);
        int32_t tot = fn->accumulator_total(&a);
        printf("accumulator_total(a) = %" PRId32 "\n", tot);
        a.methods->hicc_destroy(a);
    }

    printf("Cross-crate foo_bar_baz example passed!\n");
    return 0;
}
