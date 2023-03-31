//! `Rc<T>`, the Reference Counted Smart Pointer.
//!
//! It keeps track of the number of references to a value
//! which determines whether or not a value is still in use.
//! If there are zero references to a value, the value can be
//! cleaned up without any reference becoming valid.
//!
//! Note that `Rc<T>` is only for use in single-threaded
//! scenarios.
use std::rc::Rc;
use crate::List::{Cons, Nil};
use std::borrow::Borrow;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

/// Using Rc<T> to share data
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

pub fn list_rc() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a)); // Rc::clone() only increments the reference count.
    let c = Cons(4, Rc::clone(&a));
    // *a.borrow() += 10;
}
/// Cloning an `Rc<T>` increases the reference count
pub fn count_rc() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a)); // 1
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a)); // 2
    {
        let c = Cons(3, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a)); // 3
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a)); // 2
}