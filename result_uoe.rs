// requires: match r { Ok(_) => true,
//     Err(x) => f |=! |arg| { requires: arg == outer(x) } }
// ensures: match old(r) {
//     Ok(x) => result == x
//     Err(x) => old(f) (x) ~~> { outer(result) == result }
// }
fn unwrap_or_else<T, E, F: FnOnce (E) -> T>
       (r: Result<T, E>, f: F) -> T {
    match r {
        Ok(x) => x,
        Err(x) => f(x)
    }
}

fn main() {
    let err = Err(42);
    assert_eq! ((42, 42),
        unwrap_or_else(err, |x| (x, x)));
}
