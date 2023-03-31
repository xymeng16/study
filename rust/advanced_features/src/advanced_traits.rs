use std::path::Iter;

// associated types
struct Counter {
    data: u32
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.data < 3 {
            self.data += 1;
            Some(self.data)
        } else {
            None
        }
    }
}

//default generic type parameters and operator overloading
use std::ops::{Add, Deref};

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

/// The definition of Add trait
/// ```
/// trait Add<Rhs=Self> {
///     type Output;
///
///     fn add(self, rhs: Rhs) -> Self::Output;
/// }
///```
/// `Rhs=Self` is the default type parameters. You can leave as it
/// or use your own by specifying the Rhs type.
impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

pub fn test_point_add() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
}

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, rhs: Meters) -> Self::Output {
        Millimeters(self.0 + (rhs.0 * 1000))
    }
}

// Calling method with the same name
trait Pilot {
    fn fly(&self);
    fn fly2();
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }

    fn fly2() {
        println!("Pilot::fly2()");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
    fn fly2() {
        println!("fly2");
    }
}

pub fn test_same_name() {
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    <Human as Wizard>::fly(&person);
    Human::fly2();
    <Human as Pilot>::fly2();
    person.fly();
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

// use supertraits to require one trait's functionality within another trait
use std::fmt;
use std::fmt::Formatter;
use std::borrow::Borrow;

// OutlinePrint trait requires Display trait, hence to_string() is guaranteed to be implemented
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Point2 {
    x: i32,
    y: i32,
}

impl fmt::Display for Point2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point2 {}

pub fn test_point2_outline_print() {
    let p = Point2 {x: 1, y: 1};

    p.outline_print();
}

// use the newtype pattern to implement external traits on external types
struct Wrapper(Vec<String>);

// Display cannot be implemented on Vec since Vec is defined outside current
// scope.
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

impl Deref for Wrapper {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn test_wrapper() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
    println!("w.as_slice() = {:?}", w.as_slice());
}

