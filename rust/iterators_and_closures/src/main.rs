/*
Closure:  Anonymous functions that can capture their environment
Rust's closures are anonymous functions that can be saved into a
variable or passed as arguments to other functions. You can create
the closure in one place and then call the closure to evaluate it
in a different context. Unlike functions, closures can capture values
from the scope in which they are defined.
 */
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;
use std::thread;
use std::time::Duration;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    // generate_workout5(simulated_user_specified_value, simulated_random_number);

    // Capturing the environment with closures
    let x = 4;

    let eq_to_x = |z| z == x; // x is captured into eq_to_x

    // the following code won't compile
    // fn eq_to_x_fn(z: i32) -> bool {
    //     z == x // can't capture dynamic environment in a fn item
    // }

    let y = 4;

    assert!(eq_to_x(y));

    /* Closures can capture value from their environment (say global variables, etc.)
    in three ways, which directly map to the three ways a function can take a parameter:
        1. taking ownership
        2. borrowing mutably
        3. borrowing immutably
    These three ways are encoded in the following three Fn traits as follows
        - FnOnce *consume* the variables it captures from its enclosing scope, known as the
        closure's environment. To consume the captured variables, the closure must take ownership
        of these variables and move them into the closure when it is defined. The Once part of
        the name represents the fact that the closure can't take ownership of the same variables
        more than once, *so it can be called only once*.
        - FnMut can change the environment because it mutably borrows values.
        - Fn borrows values from the environment immutably.

    The type of traits a closure implements is automatically inferred by Rust based on how the
    closure uses the values from the environment. All closures implement FnOnce because they can
    all be called at least once. Closures that don't move the captured variables also implement
    FnMut, and closures that don't need mutable access ot the captured variables also implement
    Fn. eq_to_x closure borrows x immutably, so it has the Fn trait.

    Hence, if you want to handle the trait by yourself, you may force the closure to take ownership
    of the values it uses in the environment by using the *move* keyword before the parameter list.
    This technique is mostly useful when passing a closure to a new thread to move the data so it's
    owned by the new thread.
     */

    // Example of using move in closure
    let x = vec![1, 2, 3];

    let eq_to_x = move |z| z == x; // what if we remove move here?
                                   // then this piece of code works

    // println!("can't use x here: {:?}", x);

    let y = vec![1, 2, 3];

    assert!(eq_to_x(y));

    // Processing a series of items with iterators
    // The iterator pattern allows you to perform some task on a sequence of items in turn.
    // An iterator is responsible for the logic of iterating over each item and determining
    // when the sequence has finished.

    // In Rust, iterators are lazy, meaning they have no effect until you call method that
    // consume the iterator to use it up.
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter(); // up to now, the piece of code doesn't do anything useful.

    for val in v1_iter {
        println!("Got: {}", val);
    }

    // All iterators implement a trait named Iterator that is defined in the standard library.
    /*
        pub trait Iterator {
        type Item;

        fn next(&mut self) -> Option<Self::Item>;

        // methods with default implementations elided
    }
         */
    // The Iterator trait only requires implementors to define one method: the next method,
    // which returns one item of the iterator at a time wrapped in Some and, when iteration
    // is over, returns None. Note that calling the next method on an iterator changes internal
    // state that the iterator uses to keep track of where it is in the sequence. Hence the
    // iterator should be mut.
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

    // Consuming adaptors: methods that call next()
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum(); // this is a consuming adaptor

    assert_eq!(total, 6);

    // we aren't allowed to use v1_iter after the call to sum because sum takes ownership of
    // the iterator we call it on

    // Iterator adaptors: methods that produce other iterators
    // Note: "But because all iterators are lazy, you have to call one of the consuming adaptor
    // methods to get results from calls to iterator adaptors."
    let v1: Vec<i32> = vec![1, 2, 3];

    v1.iter().map(|x| x + 1); // up to now, above code do nothing
    let v2: Vec<_> = v1.iter().map(|x| -> f32 {*x as f32 + 1.1}).collect(); // use collect() to consume it.
    for v in v2 {
        println!("Got: {}", v);
    }
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

// example of using filter()
fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }

    #[test]
    fn calling_next_directly() {
        let mut counter = Counter::new();

        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }

    #[test]
    fn using_other_iterator_trait_methods() {
        let sum: u32 = Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();
        assert_eq!(18, sum);
    }
}

// Create our own iterators with the Iterator trait
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter {count: 0}
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count) // continue
        } else {
            None // end
        }
    }
}

// Creating an abstraction of behavior
fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    return intensity;
}

fn generate_workout(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            simulated_expensive_calculation(intensity)
        );
        println!(
            "Next, do {} situps!",
            simulated_expensive_calculation(intensity)
        )
    } else {
        if random_number == 3 {
            println!("Take a break today!  Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                simulated_expensive_calculation(intensity)
            );
        }
    }
}

fn generate_workout2(intensity: u32, random_number: u32) {
    let expensive_result = simulated_expensive_calculation(intensity);
    // This variable saves the intensity related value (to save computation)

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result);
        println!("Next, do {} situps!", expensive_result);
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result);
        }
    }
}

fn generate_workout3(intensity: u32, random_number: u32) {
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        return num;
    }; // this expression defines a closure, with an argument "num"

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure(intensity));
        println!("Next, do {} situps!", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_closure(intensity));
        }
    }
} /*
  This is okay, but still not efficient enough, just like generate_workout(), it compute
  the value twice in the first if block, we could fix this problem by creating a variable
  local to that if block to hold the result of calling the closure, but closure provide
  us with another solution. This will be talked later. Now let's focus on why there isn't
  a type annotations in the closure definition and the traits involved with closure.
  */

fn closure_type_inference_and_annotation() {
    /*
    Closure's type inference mechanism allows you to omit the type annotation of the
    parameters and the return value.
    Type annotation are required by function definitions because they are part of the
    explict exposed interfaces to users, of which we assume that user may use them in
    an incorrect manner. Hence such annotation is very important to ensure that everyone
    agrees on what types of values a function uses and returns and tell the compiler how
    to check the code.
    Also, closures are usually short and relevant only within a narrow context rather than
    in any arbitrary scenario. Within these limited contexts, the compiler is reliably able
    to infer the types of the parameters and the return type, similar to how it’s able to
    infer the types of most variables.
    Making programmers annotate the types in these small, anonymous functions would be
    annoying and largely redundant with the information the compiler already has available.
     */
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        return num;
    }; // This is an example of closure definition with explicit type annotation (optional)

    fn add_one_v1(x: u32) -> u32 {
        return x + 1;
    }
    let add_one_v2 = |x: u32| -> u32 {
        return x + 1;
    };
    // let add_one_v3 = |x| { return x + 1;};
    // let add_one_v4 = |x| return x+1;; // Calling v3 and v4 is required to be compiled
    // since each closure need to have an clear type, either annotated or inferred

    // But... Is the type of closure dynamic?
    let dyn_or_not = |x| x;

    let s = dyn_or_not(String::from("hello"));
    // let n = dyn_or_not(5); // This won't compiled since at the first call of the closure, it is
    // inferred that the type of x is String
}

// Storing closures using generic parameters and the Fn traits
// Here we create a struct that will hold the closure and the
// resulting value of calling the closure. The struct will execute
// the closure only if we need the resulting value, and it will
// cache the resulting value so the rest of our code doesn't have
// to be responsible for saving and reusing the result. This is
// the so-called memoization or lazy-evaluation. Let's see how to
// achieve that.
struct Cacher<T>
where
    T: Fn(u32) -> u32, // the grammar candy of <T: Fn(u32) -> u32>
{
    calculation: T,
    // Any closure we want to store in the calculation field
    // must have one u32 parameter and must return a u32
    value: Option<u32>, // None or u32
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(x) => x, // already been calculated
            None => {
                let v = (self.calculation)(arg); // do the calculation
                self.value = Some(v); // save to the value field
                v // return
            }
        }
    }
}

fn generate_workout4(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

// Limitation of Cacher implementation:
// 1. The value won't change even if the parameter changes
// How to fix: introduce a HashMap into the Cacher
// 2. Up to now, it only takes u32 variable as its parameter, more generic parameters
// are needed to be introduced to increase the flexibility of the Cacher functionality
struct Cacher2<K, T>
// To accept more types, we use another generic parameters K here
// to represent the type of the parameter
where
    K: Eq + Hash + Copy + Display, // For a type that could be stored into a HashMap,
    // it must implement Eq and Hash trait, to print it, Display trait must be implemented
    // to invoke the Hashmap::entry(key: K) method without moving the value of key, Copy
    // trait must be implemented
    T: (Fn(K) -> K), // K is the type of the parameter and the return value in Fn trait
{
    calculation: T,
    hashmap: HashMap<K, K>, // To check if the parameter changes, we use HashMap instead of Option<K>
}

impl<K, T> Cacher2<K, T>
where
    K: Eq + Hash + Copy + Display,
    T: (Fn(K) -> K),
{
    fn new(calculation: T) -> Cacher2<K, T> {
        Cacher2 {
            calculation,
            hashmap: HashMap::new(),
        }
    }

    fn value(&mut self, arg: K) -> K {
        let val: &K = match self.hashmap.entry(arg) {
            // try to get the entry for the corresponding key
            Entry::Occupied(o) => o.into_mut(), // the entry is occupied, getting the mut ref of the value
            Entry::Vacant(v) => v.insert((self.calculation)(arg)), // the entry is vacant, do the calculation
        };
        *val
    }
}

fn generate_workout5(intensity: usize, random_number: u32) {
    let mut expensive_result = Cacher2::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity + 1));
        println!("Today, do {} pushups!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}
