use prusti_contracts::*;

// Prusti glue for Vec<i32>
    #[trusted]
    #[ensures(vec_len(&result) == 0)]
    fn vec_new() -> Vec<i32> {
        vec![]
    }

    #[pure]
    #[trusted]
    #[ensures(result >= 0)]
    fn vec_len(vec: &Vec<i32>) -> usize {
        vec.len()
    }

    #[pure]
    #[trusted]
    #[requires(idx >= 0 && idx < vec_len(vec))]
    #[ensures(vec_contains(vec, result))]
    fn vec_lookup(vec: &Vec<i32>, idx: usize) -> i32 {
        vec[idx]
    }

    #[pure]
    #[trusted]
    fn vec_contains(vec: &Vec<i32>, val: i32) -> bool {
        vec.contains(&val)
    }

    #[trusted]
    #[ensures(vec_len(vec) == old(vec_len(vec)) + 1)]
    #[ensures(
        forall(|idx: usize| 0 <= idx && idx < old(vec_len(vec))
            ==> vec_lookup(vec, idx) == old(vec_lookup(vec, idx)),
            triggers = [(vec_lookup(vec, idx),)])
    )]
    #[ensures(vec_lookup(vec, old(vec_len(vec))) == value)]
    #[ensures(vec_contains(vec, value))]
    fn vec_push(vec: &mut Vec<i32>, value: i32) {
        vec.push(value);
    }
// end Prusti glue

#[requires(f |= |arg: i32| -> [ requires(vec_contains(v, arg)) ])]
#[ensures(
    result ==>
        forall(|idx: usize| 0 <= idx && idx < vec_len(v)
            ==> f ~>! |arg: i32| -> { arg == vec_lookup(v, idx) } { cl_result })
)]
#[ensures(
    !result ==> exists(|idx: usize| 0 <= idx && idx < vec_len(v)
        && f ~>! |arg: i32| -> { arg == vec_lookup(v, idx) } { !cl_result })
)]
fn all_vec<T: Fn(i32) -> bool>(v: &Vec<i32>, f: T) -> bool {
    let mut i = 0;
    while i < vec_len(v) {
        body_invariant!(i >= 0 && i < vec_len(v));
        body_invariant!(
            forall(|idx: usize| 0 <= idx && idx < i
                ==> f ~>! |arg: i32| -> { arg == vec_lookup(v, idx) } { cl_result })
        );
        let el = vec_lookup(v, i);
        if !f(el) {
            return false;
        }
        i += 1;
    }
    return true;
}

fn main() {
    // let v = vec![1, 2, 3, 4];
    let mut v = vec_new();
    vec_push(&mut v, 1);
    vec_push(&mut v, 2);
    vec_push(&mut v, 3);
    vec_push(&mut v, 4);

    // sanity check
    assert!(vec_len(&v) == 4);
    assert!(vec_lookup(&v, 0) == 1);
    assert!(vec_lookup(&v, 3) == 4);

    assert!(!all_vec(
        &v,
        closure!(
            #[requires(true)]
            #[ensures(result == (i == 4))]
            |i: i32| -> bool { i == 4 }
        ),
    ));
    assert!(all_vec(
        &v,
        closure!(
            #[requires(true)]
            #[ensures(result == (i < 5))]
            |i: i32| -> bool { i < 5 }
        ),
    ));
    assert!(!all_vec(
        &v,
        closure!(
            #[requires(true)]
            #[ensures(result == (i < 3))]
            |i: i32| -> bool { i < 3 }
        ),
    ));
}
