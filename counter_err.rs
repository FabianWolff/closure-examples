// From Kassios and Müller (2010) (simplified)

#[requires(f |= || { ensures(result >= 1) })]
fn foo(f: impl FnMut() -> i32) {}

fn main() {
    let mut x = 0;
    let mut inc =
        #[invariant(x >= 0)]
        #[invariant(old(x) <= x)]
        #[ensures(old(x) + 1 == x)]
        || { let r = x; x += 1; r };

    foo(inc); // <-- precondition (foo's) violation here
}
