//! Rust allows memory leaks by using Rc<T> and RefCell<T>: it is possible to create references where
//! items refer to each other in a cycle. The reference count of each item in the cycle will never
//! reach 0, hence values will never be dropped.

use std::cell::{Ref, RefCell};
use std::rc::{Rc, Weak};

use crate::List::{Cons, Nil};
use std::borrow::BorrowMut;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    // the pointer is mutable but the i32 value is not
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

pub fn ref_cycle() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        // get the refcell of the a's tail if a is not Nil
        *link.borrow_mut() = Rc::clone(&b); // set a's next as b
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing b = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());
}

// preventing reference cycles: turing an Rc<T> into a Weak<T>
// example: Creating a tree data structure: a node with child nodes

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
    // we cannot use Rc here to avoid reference cycle
    // notice the logic that if parent is destroyed, children should be done so (strong count = 0)
    // but the status of children shouldn't influence the parent
    // here we use Weak instead of Rc
}

// pub fn simple_tree() {
//     let leaf = Rc::new(Node {
//         value: 3,
//         children: RefCell::new(vec![]),
//     });
//
//     let branch = Rc::new(Node {
//         value: 4,
//         children: RefCell::new(vec![Rc::clone(&leaf)]),
//     });
//
//     // we can get from branch to leaf since we have pointer to leaf in branch,
//     // but we cannot get from leaf to branch.
//     // we want leaf to know that branch is its parent
// }

pub fn make_tree() {
    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
        parent: RefCell::new(Weak::new()),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade()); // upgrade returns an Option

    let branch = Rc::new(Node {
        value: 5,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
        parent: RefCell::new(Weak::new()),
    });

    // we can get from branch to leaf since we have pointer to leaf in branch,
    // but we cannot get from leaf to branch.
    // we want leaf to know that branch is its parent
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade()); // upgrade returns an Option
}

pub fn make_tree2() {
    println!("---make_tree2()---");
    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
        parent: RefCell::new(Weak::new()),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade()); // upgrade returns an Option
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}