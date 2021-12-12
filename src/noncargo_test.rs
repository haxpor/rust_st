/// Demonstrate how to emulate `cargo test` for non-cargo project.
/// Inspired by https://users.rust-lang.org/t/running-cargo-test-without-cargo/31837/2?u=haxpor.
/// Steps:
/// - rustc --crate-name foo --crate-type lib noncargo_test.rs
/// - rustc --crate-name foo noncargo.rs --test
/// - ./foo # run test binary

fn dummy_add(x: u64, y: u64) -> u64 {
    x + y
}

#[test]
fn tst_dummy_add() {
    assert_eq!(2, dummy_add(1, 1));
}
