/*
The structure of this library:
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
The entire module tree is rooted under the implicit module named crate.
*/
mod front_of_house {
    // define a module
    pub mod hosting {
        // nest module
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // serve_order(); // not work, since serve_order is not in the scope of back_of_house
        super::serve_order(); // works!
                              // relative path provides you with great portability when you want to move the crate to
                              // somewhere else
    }
    fn cook_order() {}

    /*
    我们定义了一个公有结构体 back_of_house:Breakfast，
    其中有一个公有字段 toast 和私有字段 seasonal_fruit。
    这个例子模拟的情况是，在一家餐馆中，顾客可以选择随餐附赠的面包类型，
    但是厨师会根据季节和库存情况来决定随餐搭配的水果。
    餐馆可用的水果变化是很快的，所以顾客不能选择水果，
    甚至无法看到他们将会得到什么水果。
    */
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String, // private as default
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            // borrow the ownership from the passed parameter toast
            // here summer perform as a constructor of Breakfast,
            // because seasonal_fruit is private and without a constructor
            // we cannot assign it a value
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // unlike struct, once we set a enum as public, 
    // all of the elements belonging to it will be public
    pub enum Appetizer {
        Soup,
        Salad,
    }
}
use front_of_house::hosting;
use front_of_house::hosting::add_to_waitlist;
use std::collections::HashMap;
// the path to a function, example:
pub fn eat_at_restaurant() {
    let mut map = HashMap::new();
    map.insert(1, 2);
    
    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // relative path
    front_of_house::hosting::add_to_waitlist();

    /*
        when hosting is not pub, above code is not able to be compiled!
        error[E0603]: module `hosting` is private
      --> src\lib.rs:35:28
       |
    35 |     crate::front_of_house::hosting::add_to_waitlist();
       |                            ^^^^^^^ private module
       |
    note: the module `hosting` is defined here
      --> src\lib.rs:16:5
       |
    16 |     mod hosting {
       |     ^^^^^^^^^^^


    error[E0603]: module `hosting` is private
      --> src\lib.rs:38:21
       |
    38 |     front_of_house::hosting::add_to_waitlist();
       |                     ^^^^^^^ private module
       |
    note: the module `hosting` is defined here
      --> src\lib.rs:16:5
       |
    16 |     mod hosting {
       |     ^^^^^^^^^^^
    */

    // hence the hosting module must be set to public (privacy boundary)
    // in rust, all of the elements are private in default,
    // parent cannot use private elements from its child but child can

    /*
    even though we set hosting as pub, it is not enough for us to invoke
    above functions since they are still private. so we need to set the as
    pub too!
    */

    // Order a breakfast in the summer with Whole Wheat
    let mut meal = back_of_house::Breakfast::summer("Whole Wheat");
    // Change our mind of what bread to eat
    meal.toast = String::from("Wheat"); // kinda unhealthy :(
    println!("I'd like {} toast please!", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
