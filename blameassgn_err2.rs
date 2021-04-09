// From Findler and Felleisen (2002)

#[requires(f |= |i| { requires(i > 9),
                      ensures(result >= 0 && result <= 99) })]
#[ensures(result >= 0 && result <= 99)]
fn g(mut f: impl FnMut(i32) -> i32) -> i32 {
    f(10)
    // f(0) should cause a verification error
}

fn main() {
    let h =
        #[requires(i > 9)]
        #[ensures(result >= 50 && result <= 99)]
        |i: i32| { 66 };
    g(h);

    let f =
        #[requires(i > 9)]
        #[ensures(result >= -10 && result <= 89)]
        |i: i32| { -10 };
    g(f); // <-- precondition (g's) violation here
}
