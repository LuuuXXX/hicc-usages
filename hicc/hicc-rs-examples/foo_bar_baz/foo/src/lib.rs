#![feature(specialization)]

hicc_rs::foreign!();

use hicc_rs::*;

// Import everything through type aliases defined in bar & baz
use example_foo_bar_baz_bar::{Line, MyAccumulator, MyCounter, MyInt, MyPoint};
use example_foo_bar_baz_baz::{MyDouble, MyGreeter, MyRect};

fn add_points(p1: MyPoint, p2: MyPoint) -> MyPoint {
    MyPoint {
        x: p1.x + p2.x,
        y: p1.y + p2.y,
    }
}

fn make_point(x: MyInt, y: MyInt) -> MyPoint {
    MyPoint { x, y }
}

fn make_line(start: MyPoint, end: MyPoint) -> Line {
    Line { start, end }
}

fn rect_area(r: MyRect) -> MyDouble {
    r.width * r.height
}

fn make_rect(w: MyDouble, h: MyDouble) -> MyRect {
    MyRect {
        width: w,
        height: h,
    }
}

fn new_counter(initial: i64) -> MyCounter {
    MyCounter::new(initial)
}

fn counter_inc(c: &mut MyCounter, delta: i64) -> i64 {
    c.increment(delta);
    c.get()
}

fn counter_get(c: &MyCounter) -> i64 {
    c.get()
}

fn new_accumulator() -> MyAccumulator {
    MyAccumulator::new()
}

fn accumulator_add(c: &mut MyAccumulator, val: MyInt) -> MyInt {
    c.add(val);
    c.total()
}

fn accumulator_total(c: &MyAccumulator) -> MyInt {
    c.total()
}

fn new_greeter(greeting: String) -> MyGreeter {
    MyGreeter::new(greeting)
}

fn greet(g: &MyGreeter, name: &str) -> String {
    g.greet(name)
}

fn greeter_set(g: &mut MyGreeter, greeting: String) {
    g.set_greeting(greeting);
}

#[export_lib(foreign, name = "foo_bar_baz")]
mod lib {
    use super::*;

    fn add_points(p1: MyPoint, p2: MyPoint) -> MyPoint;
    fn make_point(x: MyInt, y: MyInt) -> MyPoint;
    fn make_line(start: MyPoint, end: MyPoint) -> Line;

    fn rect_area(r: MyRect) -> MyDouble;
    fn make_rect(w: MyDouble, h: MyDouble) -> MyRect;

    fn new_counter(initial: i64) -> MyCounter;
    fn counter_inc(c: &mut MyCounter, delta: i64) -> i64;
    fn counter_get(c: &MyCounter) -> i64;

    fn new_accumulator() -> MyAccumulator;
    fn accumulator_add(c: &mut MyAccumulator, val: MyInt) -> MyInt;
    fn accumulator_total(c: &MyAccumulator) -> MyInt;

    fn new_greeter(greeting: String) -> MyGreeter;
    fn greet(g: &MyGreeter, name: &str) -> String;
    fn greeter_set(g: &mut MyGreeter, greeting: String);
}
