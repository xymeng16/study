use std::fmt::{Debug, Display};
use std::iter::Sum;

fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    return largest;
}

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_generic<T: std::cmp::PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0]; // cannot move since there is no guarantee that T implements Copy trait

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    return largest;
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        return &self.x;
    }
}

impl Point<f32> {
    fn distance_From_origin(&self) -> f32 {
        return (self.x.powi(2) + self.y.powi(2)).sqrt();
    }
}

struct Point2<T, U> {
    x: T,
    y: U,
}

// trait
pub trait Summary {
    fn summarize(&self) -> String {
        return String::from("(Read more...)");
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        return format!("{}, by {} ({})", self.headline, self.author, self.location);
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        return format!("{}: {}", self.username, self.content);
    }
}

pub struct XXArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for XXArticle {}

// Traits as parameters
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify3<T: Summary>(item1: &T, item2: &T) {
    // item1 and item2 have the same type
    println!("Breaking news! {}", item1.summarize());
}

pub fn notify4(item: &(impl Summary + Display)) {}

pub fn notify5<T: Summary + Display>(item: &T) {}

fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    return 0;
}

fn some_function2<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    return 0;
}

// returning types that implement traits
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

// define a Pair struct that implements compare trait
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    // for evert type T
    fn new(x: T, y: T) -> Self {
        return Self { x, y };
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    // for types T that impls Display and PartialOrd
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// impl trait for types that impl some traits......
// impl<T: Display> ToString for T {
//     fn to_string(&self) -> String {
//         todo!()
//     }
//     // ....
// }

// won't work since this returns different types
// error[E0308]: `if` and `else` have incompatible types
// fn returns_summarizable2(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle {
//             headline: String::from("Penguins win the Stanley Cup Championship!"),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from(
//                 "The Pittsburgh Penguins once again are the best \
//                  hockey team in the NHL.",
//             ),
//         }
//     } else {
//         Tweet {
//             username: String::from("horse_ebooks"),
//             content: String::from("of course, as you probably already know, people"),
//             reply: false,
//             retweet: false,
//         }
//     }
// }

// lifetime: to prevent dangling references
// fn dangling() {
//     let r;
//
//     {
//         let x = 5;
//         r = &x; // `x` does not live long enough
//     }
//
//     println!("r: {}", r);
// }
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        return x;
    } else {
        return y;
    }
}

// fn longest2<'a>(x: &'a str, y: &'a str) -> &'a str {
//     String::from("dllm").as_str() // returns a value referencing data owned by the current function
// }

// for struct that holds references we need to add a lifetime annotation to every reference of in the definition
struct ImportantExcerpt<'a> {
    part: &'a str,
}
// lifetime for methods
// lifetime names for struct fields always need to be declared after the `impl` keyword and then used after the struct's name
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        return 3;
    }

    // lifetime deducing rule three
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        return self.part;
    }
}
fn main() {
    // let number_list = vec![34, 50, 25, 100, 65];
    //
    // let result = largest(&number_list);
    // println!("The largest number is {}", result);
    //
    // let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    //
    // let result = largest(&number_list);
    // println!("The largest number is {}", result);

    let mut integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 }; // Automatically deduce the type of T

    // let wont_work = Point {x: 1, y: 4.0}; // won't work since x and y should have the same type
    let work = Point2 { x: 1, y: 4.0 };

    println!("{} {}", &work.x, &work.y);

    println!("{} {}", integer.x(), integer.y);
    println!("{} {}", integer.x(), integer.y);

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    let xx = XXArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };
    println!("1 new tweet: {}", tweet.summarize());
    println!("xx: {}", xx.summarize());

    let str1 = "fuck";
    let str2 = "shitty";

    println!("{} is the longest string.", longest(str1, str2));
    let x = String::from("sss");
    let res;
    {
        let y = String::from("zz");
        res = longest(x.as_str(), y.as_str());
        println!("{}", res);
    }
    let novel = String::from("Call me Ishmael. Some years ago...");
    let mut i = ImportantExcerpt { part: "" };
    {
        let first_sentance = novel.split('.').next().expect("Could not find a '.'");
        i.part = first_sentance;
    }
    println!("{}", i.part);

    // static lifetime
    let s: &'static str = "I have a static lifetime."; // the lifetime is the entire program lifetime

}
