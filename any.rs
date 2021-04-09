#[ghost_arg(P: i32 -> bool)]
#[requires(f |= |i| { requires(outer(v).contains(*i)),
                      ensures(result <==> P(*i)) })]
#[ensures(result <==> exists idx: usize :: 0 <= idx && idx < v.len() && P(v[idx]))]
fn any_vec(v: &Vec<i32>, f: impl Fn (&i32) -> bool) -> bool {
    for ref i in v {
        if f(i) {
            return true;
        }
    }
    return false;
}

fn main() {
    let v = vec![1, 2, 3, 4];
    assert!(/* P(i) := i % 2 == 0 */ any_vec(&v, |i| *i % 2 == 0));
    assert!(! /* P(i) := i > 10 */ any_vec(&v, |i| *i > 10));
}
