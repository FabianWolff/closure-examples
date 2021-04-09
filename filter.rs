// ghost_arg: P: &T -> bool
// requires: f |= |x| { ensures: result <==> P(x) }
// ensures: forall el: T ::
//     (result.contains(&el)) <==>
//     (v.contains(&el) && P(&el))
fn filter_vec<T: Copy> (v: &Vec<T>,
        mut f: impl FnMut (&T) -> bool) -> Vec<T> {
    let mut r = vec![];
    for el in v {
        if f(el) {
            r.push(*el);
        }
    }
    r
}

fn main() {
    let v = vec![0, 1, 2, 3];

    let cl =
        // ensures: result <==> *i % 2 == 0
        |i: &i32| { *i % 2 == 0 };

    let r = /* P(i) := *i % 2 == 0 */ filter_vec(v, cl);

    assert!(r.contains(2));
    assert!(!r.contains(3));
}
