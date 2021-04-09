#[ensures(result |= || { ensures(result >= 0) })]
fn foo() -> impl FnMut () -> i32 { || 42 }

fn main() {
    let mut cl = foo();
    let x = cl();
    assert!(x >= 0);
}
