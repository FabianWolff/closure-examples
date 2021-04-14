// requires: f |= |i| { requires(outer(v).contains(*i)) }
// ensures: forall idx: usize :: idx < v.length()
//   ==> :f(:i) { outer(old(v[idx])) == *i } ~~> { outer(v[idx]) == *i }
fn for_each (v: &mut Vec<i32>, mut f: impl FnMut(&mut i32) -> ()) {
    for i in v {
        f(i);
    }
}

fn main () {
    let cl =
        // ensures: *i == old(*i) + 1
        |i: &mut i32| { *i += 1; };

    let mut nums = vec![1, 2, 3];
    for_each (&mut nums, cl);
    assert_eq!(nums, vec![2, 3, 4]);
}
