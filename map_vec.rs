// requires: f |= |arg| { requires: outer(v.contains(arg)) }
// ensures: v.len() == old(v.len())
// ensures: forall i: usize :: {v[i]} {result[i]}
//   i < v.len() ==>
//     :f (:i) / { *i == outer(old(v[i])) } ~~>
//         { *i == outer(v[i]) && result == outer(result[i]) }
fn map_vec<T, U> (v: &mut Vec<T>, mut f: impl FnMut (&mut T) -> U)
        -> Vec<U> {
    let mut r = vec![];
    r.reserve_exact(v.len());

    for el in v {
        r.push (f (el));
    }

    r
}

fn main() {
    let mut v = vec![0, 1, 2];
    let cl =
        // ensures: *i == old(*i) + 1 && result == old(*i) * 2
        |i: &mut i32| -> i32 { let r = *i * 2; *i += 1; r };

    let r = map_vec(&mut v, cl);

    assert_eq!(v[2], 3);
    assert_eq!(r[2], 4);
}
