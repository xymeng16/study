//! This crate talks about `RefCell<T>` and the interior mutability pattern.
//! Interior mutability is a design pattern in Rust that allows you to mutate
//! data even when there are immutable references to that data, which is
//! disallowed by the borrowing rules. To mutate data, the pattern uses `unsafe`
//! code inside a data structure to bend Rust's usual rules that govern mutation
//! and borrowing. We can use types that use the interior mutability pattern when
//! we can ensure that the borrowing rules will be followed at runtime, even though
//! the compiler can't guarantee that. The `unsafe` code involved is then wrapped
//! in a safe API, and then outer type is still immutable. We can implement this
//! feature by using `RefCell<T>` type which follows the interior mutability.
//!
//! Some difference between types representing single ownership over data it holds
//! and `RefCell<T>`:
//! - `RefCell<T>` enforces the borrowing rules' invariants at runtime
//! `Refcell<T>` is only used for single-thread scenarios and will give you a
//! compile-time error if you try using it in a multithreaded context.
//!
//! Interior mutability pattern: mutating the value inside an immutable value
use std::cell::RefCell;

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                // sent_messages: vec![],
                sent_messages: RefCell::new(vec![]),
            }

        }
    }

    impl Messenger for MockMessenger {
        /// for the first `&self` parameter, it is still an immutable borrow of `self`, which matches the
        /// definition of the trait. And borrow_mut on the `RefCell<Vec<String>>` is called to get a
        /// mutable reference to the value inside the `RefCell<Vec<String>>`
        fn send(&self, message: &str) {
            // self.sent_messages.push(String::from(message)); // can not compile since self is not mutable
            self.sent_messages.borrow_mut().push(String::from(message));
            // by using RefCell, you can borrow an mutable reference from an immutable reference
        }
    }
    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messager = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messager, 100);

        limit_tracker.set_value(80);
        assert_eq!(mock_messager.sent_messages.borrow().len(), 1);
    }
}

// Interior Mutability: A mutable borrow to an immutable value
fn not_work() {
    let x = 5;
    // let y = &mut x; // x is not mutable hence cannot be borrowed as mutable
} // What if we'd like make x mutable inside this function but immutable
// outside?

// we’ll create a library that tracks a value against a maximum value and sends
// messages based on how close to the maximum value the current value is. This
// library could be used to keep track of a user’s quota for the number of API
// calls they’re allowed to make, for example.
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!"); // send takes an immutable
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        }
        else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used uo over 75% of your quota!");
        }
    }
}

// When creating immutable and mutable references,we use & and &mut syntax, respectively.
// With RefCell<T>, we use the borrow and borrow_mut methods, which are part of the safe
// APU that belongs to RefCell<T>. (do ownership check at runtime). The borrow() method
// returns the smart pointer type Ref<T>, and borrow_mut() returns the smart pointer type
// RefMut<T>. Both types implement Deref, so we can treat them like regular references.

// Having multiple owners of mutable data by combining Rc<T> and RefCell<T>
// Rc<T> allows you to have multiple owners of some data, but only gives immutable
// access to that data. Hence if you have an Rc<T> that holds a RefCell<T>, you can get a
// value that can have multiple owners and that you can mutate!


#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}
use std::rc::Rc;
use crate::List::{Cons, Nil};

pub fn refcell_in_rc() {
    // Cons(ref i32, next List)
    let value = Rc::new(RefCell::new(5)); // this is the value in the last node

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}