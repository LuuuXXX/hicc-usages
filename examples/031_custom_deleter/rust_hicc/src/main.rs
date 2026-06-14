use custom_deleter::IntArray;

fn main() {
    let arr = IntArray::new(5);
    for i in 0..arr.size() {
        println!("arr[{}]={}", i, arr.read_at(i));
    }
    println!("status={}", custom_deleter::custom_deleter_status());
}
