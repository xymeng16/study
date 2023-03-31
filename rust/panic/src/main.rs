use std::fs::File;
use std::io;
use std::io::{ErrorKind, Read};
use std::fs;

fn main() {
    // Try to recover from errors
    /*
    use
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
     */

    // let f = File::open("hello.txt");

    // The following lines will panic no matter why File::open failed.
    // let f = match f {
    //     Ok(file) => file, // return file if okay
    //     Err(error) => {
    //         panic!("Problem opening the file: {:?}", error)
    //     },
    // };

    // The following lines will panic based on different kinds of errors
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("file create failed: {:?}", e),
    //         },
    //         other_error => panic!("file open failed: {:?}", other_error),
    //     },
    // };

    // let f = File::open("hello.txt").unwrap(); // return the value in ok if succeeded
    let f = File::open("hello.txt").expect("failed to open hello.txt"); // customize the panic message
}

fn read_username_from_file() -> Result<String, io::Error> {
    // decode the Result by the callee
    let f = File::open("hello.txt");

    let mut f = match f {
        // check file open status
        Ok(file) => file,
        Err(e) => return Err(e), // propagate to the calling code
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        // check file reading status
        Ok(_) => Ok(s),
        Err(e) => Err(e), // propagate..
    }
}

/*
The ? placed after a Result value is defined to work in almost the same way as the match expressions
we defined to handle the Result values before.
If the value of the Result is an Ok ,the value inside the Ok will get returned from this expression,
and the program will continue. If the value is an Err, the Err will be returned from the whole function
as if we had used the return keyword so the error value gets propagated to the callee.

Note that ? operator take cares of the Error type conversion automatically by the from function defined
in the from trait.
(What a tricky design)
 */
fn read_username_from_file_shortcut() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    return Ok(s);
}

fn read_username_from_file_even_shorter() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    return Ok(s);
}

fn read_user_from_file_even_much_shorter() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}