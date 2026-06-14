use operator_overload::*;

fn main() {
    let a = Vec2::new(1.0, 2.0);
    let b = Vec2::new(3.0, 4.0);

    let c = vec_add(&a, &b);
    let d = vec_sub(&a, &b);
    let e = vec_scale(&a, 2.0);
    let f = vec_neg(&a);
    println!("a=({},{}) b=({},{})", a.x(), a.y(), b.x(), b.y());
    println!("a+b=({},{})  a-b=({},{})  a*2=({},{})  -a=({},{})",
        c.x(), c.y(), d.x(), d.y(), e.x(), e.y(), f.x(), f.y());

    let mut g = Vec2::new(0.0, 0.0);
    vec_iadd(&mut g, &a);
    println!("g+=a -> ({},{})", g.x(), g.y());

    println!("a==b? {}", vec_eq(&a, &b));
    println!("a[0]={} a[1]={}", vec_at(&a, 0), vec_at(&a, 1));
}
