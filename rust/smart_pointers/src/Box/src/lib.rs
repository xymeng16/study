use crate::List::{Cons, Nil};
use std::ops::Deref;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub fn store_on_heap() {
    let b = Box::new(5);
    println!("b = {}", b);
}

// Example of defining a con list with Box
enum List {
    Cons(i32, Box<List>),
    Nil,
}

pub fn con_list() {
    // Boxes provide only the indirection and heap allocation
    // When a Box<T> value goes out of scope, the heap data
    // that the box is pointing to is cleaned up as well because
    // of the Drop trait implementation
    let _ = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

// treating smart pointers like regular references with the Deref trait
pub fn deref_example() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

pub fn deref_box() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

// defining our own smart pointer
struct MyBox<T>(T); // The MyBox type is a tuple struct with one element of type T

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn deref_mybox() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

// implicit deref coercions with functions and methods
// it can convert &String to &str because String implements
// the Deref trait such that it returns str.
// Deref coercion happens automatically when we pass a reference
// to a particular type's value as an argument to a function or
// method that doesn't match the parameter type in the function or
// method definition

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

pub fn deref_coercion_mybox() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m); // &MyBox<String> -> &String -> &str
    // or explicitly deref and ref
    hello(&(*m)[..]); // (*m) is String, (*m)[..] is str, &(*m)[..] is &str
    hello((*m).as_str());
}

// How deref coercion interacts with mutability
// Rust does deref coercion when it finds types and traits
// in three cases:
// - From `&T` to `&U` when `T: Deref<Target=U>`
// - From `&mut T` to `&mut U` when `T: DerefMut<Target=U>`
// - From `&mut T` to `&U` when `T: Deref<Target=U>`