// requires: f |= || { requires: true }
// ensures: result.len() == n
// ensures: forall idx: usize :: {result[idx]}
//     idx < n ==>
//         :f () ~~> { outer(result[idx]) == result }
fn repeat_with_n<T, F: FnMut() -> T>(mut f: F, n: usize) -> Vec<T> {
    let mut r = vec![];
    r.reserve_exact(n);

    for _ in 0 .. n {
        r.push(f());
    }

    r
}

fn main() {
    let mut c = 0;
    let mut cl =
        // invariant: c >= old(c)
        // ensures: c == old(c) + 1
        // ensures: result == c * 2
        || -> i32 { c += 1; c * 2 };
    cl();

    let r = repeat_with_n(cl, 10);

    for i in r {
        assert!(i % 2 == 0 && i >= 2);
    }
}
