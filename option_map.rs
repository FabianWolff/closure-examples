// requires: match self {
//     None => true,
//     Some(x) => f |=! |arg| { requires: arg == outer(x) }
// }
// ensures: match old(self) {
//     None => result == None,
//     Some(x) => old(f) (x) ~~>
//         { outer(result) == Some(result) }
// }
pub fn map<T, U, F: FnOnce(T) -> U>(slf: Option<T>, f: F) -> Option<U> {
    match slf {
        Some(x) => Some(f(x)),
        None => None,
    }
}

fn main() {
    let mut count = 0;
    let mut cl =
        // requires: i != count
        // ensures: count == old(count) + 1
        |i: i32| -> i32 { let _r = 42 / (i - count); count += 1; };

    let a = map(Some(1), &mut cl); // works
    // let b = map(Some(1), &mut cl); // fails to verify
}
