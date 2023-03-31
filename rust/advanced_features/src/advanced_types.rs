use std::borrow::Borrow;

type Thunk = Box<dyn Fn() + Send + 'static>;

pub fn test_type_aliases() {
    let f: Thunk = Box::new(|| println!("hi!"));
    f();
}

pub fn test_never()->Vec<i32> {
    let t = Option::Some(vec![1, 2, 3]);

    match t {
        Some(s) => s,
        None => loop {
            print!("end ever")
        },
    }
}

// dynamically sized types and the sized trait
pub fn test_dyn_size() {
    let s1 = "hello";
    let s2 = "hello";

    let p1 = s1 as *const str;
    let p2 = s2 as *const str;
    let p3 = s1.as_ptr();
    let p4 = s2.as_ptr();

    println!("p1: {:?}, p2: {:?}, p3: {:?}, p4: {:?}", p1, p2, p3, p4);
}


fn generic<T: ?Sized>(t: &T) {
    // T may or may not be Sized
    // we use &T since T may not be Sized,
    // To use t, we need to use it behind some kind of pointer
}