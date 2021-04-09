// From Kassios and Müller (2010)

// requires: h |=! || { requires: true}
// requires: g |=! || { requires: true}
// ensures: *y == 4
// ensures: old(h) () ~~> { outer(h) == self }
// ensures: old(g) () ~~> { outer(g) == self }
fn f(y: &mut i32,
     h: &mut impl FnMut() -> (),
     g: &mut impl FnMut() -> ()) {
    let mut x = 3;
    *y = 4;
    h ();
    g ();
}

fn main() {
    let h = || {};

    let mut x = 42;
    let g =
        // ensures(x == old(x) + 1)
        || { x += 1; };

    let mut y = 0;

    f(&mut y, &mut h, &mut g);

    assert_eq!(y, 4);
    assert_eq!(x, 43);
}
