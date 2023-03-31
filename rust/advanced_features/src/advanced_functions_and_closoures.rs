fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

pub fn test_func_ptr() {
    // unlike closures, fn is a type rather than a trait
    // Function pointers implement all three of the closure traits
    // (Fn, FnMut, and FnOnce), so you can always pass a function
    // pointer as an argument for a function that expects a closure.
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);

    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings1: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
    let list_of_strings2: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();
    assert_eq!(list_of_strings1, list_of_strings2);
}

fn return_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}