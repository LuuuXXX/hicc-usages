use placement_new::{Buffer, Placement};

fn main() {
    let buf = Buffer::new(64);
    println!("buffer size = {}", buf.size());

    let mut mem = vec![0u8; 64];
    {
        let mut p = Placement::new(&mut mem, 42);
        println!("payload value = {}", p.value());
        p.set(100);
        println!("payload value = {}", p.value());
    }
    println!("done");
}
