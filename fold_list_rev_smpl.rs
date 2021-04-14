// From Svendsen et al. (2010) (simplified)

struct Node<T> {
    el: T,
    next: Option<Box<Node<T>>>,
}

impl<T: Copy> Node<T> {
    // ensures: els(result) == old(v)
    fn new (mut v: Vec<T>) -> Node<T> {
        Node {
            el: v.remove(0),
            next: match v.len() {
                0 => None,
                _ => Some(Box::new(Node::new(v)))
            }
        }
    }

    #[pure]
    fn els (&self) -> Vec<T> {
        let mut r = vec![self.el];
        match self.next {
            Some(ref bx) => r.append(&mut bx.els()),
            None => {}
        }
        r
    }

    #[pure]
    fn opt_els (n: &Option<Box<Self>>) -> Vec<T> {
        match n {
            Some(b) => {
                let mut r = vec![b.el];
                r.append(&mut Self::opt_els(&b.next));
                r
            }
            None => vec![]
        }
    }

    // ghost_arg: inv: (i32, F) -> bool
    // ghost_arg: l: i32
    // requires: inv(l, f)
    // requires: forall l: i32 :: {inv(l, f)}
    //     f |= |n| {
    //         requires: outer(self.els()).contains(n.el)
    //             && inv(outer(l), self)
    //         ensures: inv(outer(l) + 1, self)
    //     }
    // ensures: inv(old(self.els().len()), f)
    fn fold<F: FnMut(Box<Self>)> (mut self: Box<Self>, mut f: F) {
        let mut next = None;
        std::mem::swap (&mut next, &mut self.next);
        f (self);

        if let Some(next) = next {
            // inv := inv
            // l := l + 1
            next.fold(f);
        }
    }
}

fn main() {
    let list = Box::new(Node::new(vec! [0, 1, 2]));
    assert_eq!(list.els(), vec![0, 1, 2]);

    let mut rev_head = Box::new(None);
    // inv(l, cl) := Node::opt_els(*cl.rev_head).len() == l
    // l := 0
    list.fold(
        // view: rev_head := rev_head
        // ensures: n.el == old(n.el)
        // ensures: Node::opt_els(*rev_head).len()
        //     == 1 +  old(Node::opt_els(*rev_head).len())
        |mut n| { std::mem::swap(&mut *rev_head, &mut n.next);
                  *rev_head = Some(n); });

    assert_eq! (rev_head.map(|n| n.els().len()), Some(3));
}
