use std::env;
use std::process;

use minigrep::Config;
use minigrep::Cfg;
fn main() {
    // let args: Vec<String> = env::args().collect(); // env::args() returns an iterator of the command line arguments
    //                                                // use collect() method on an iterator to turn it into a collection (such as a Vec)
    // let config = Config::new(&args).unwrap_or_else(|err| {
    //     // use closure to handle exceptions
    //     eprintln!("Problem parsing arguments: {}", err);
    //     process::exit(1);
    // });
    let config = Cfg::new(env::args()).unwrap_or_else(|err| {
        // use closure to handle exceptions
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    // run(config); // extract logic from main function
    if let Err(e) = minigrep::run(config) {
        // an grammar candy for switch case on error handling
        // we don't need to use unwrap_or_else() since the Ok value is just the unit type (),
        // hence no unwrapping is required
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
