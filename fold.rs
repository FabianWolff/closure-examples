// ghost_arg: inv(&Vec<T>, usize, A): bool
// requires: inv(v, 0, init)
// requires: forall end: usize ::
//   end < v.len() ==>
//     f |= |a, c| { requires: inv(outer(v), outer(end), a)
//                       && outer(v[end]) == c,
//                   ensures: inv(outer(v), outer(end) + 1, result) }
// ensures: inv(v, result)
fn fold_vec<T, A> (v: &Vec<T>, init: A, mut f: impl FnMut (A, &T) -> A)
        -> A {
    let mut acc = init;
    for el in v {
        acc = f (acc, el);
    }
    acc
}

fn main() {
    let v = vec![true, false, true];

    let all = fold_vec(&v, true, |a, c| a && c);
    let any = fold_vec(&v, false, |a, c| a || c);

    assert!(!all);
    assert!(any);
}
