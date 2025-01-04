use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    next: RefCell<Option<Rc<Node>>>,
}

fn main() {
    let node1 = Rc::new(Node {
        value: 1,
        next: RefCell::new(None),
    });

    let node2 = Rc::new(Node {
        value: 2,
        next: RefCell::new(Some(Rc::clone(&node1))),
    });

    *node1.next.borrow_mut() = Some(Rc::clone(&node2));

    println!("node1 count: {}", Rc::strong_count(&node1));
    println!("node2 count: {}", Rc::strong_count(&node2));
}