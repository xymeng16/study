#[macro_export]
macro_rules! vec2 {
    ($($x:expr), *) => { // asterisk means repeat
        let mut temp_vec = Vec::new();
        $(
            temp_vec.push($x);
        )*
        temp_vec
    }
}

use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

pub fn test_procedural_macro() {
    Pancakes::hello_macro();
}