use hello_world::{answer, hello_world};

#[test]
fn answer_is_42() {
    assert_eq!(answer(), 42);
}

#[test]
fn hello_world_runs() {
    hello_world(); // 不 panic 即通过
}
