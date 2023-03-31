//! # minigrep
//!
//! `minigrep` is a tiny but useful globally regex expression print utility
//! implemented in Rust.

use std::env;
use std::error::Error;
use std::fs;

pub use self::Config as Cfg;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // () is the unit type, meaning return nothing
    // Box<dyn Error> means the function will return a type that implements the Error trait
    let contents = fs::read_to_string(config.filename)?; // ? will return the error value
                                                         // from the current function for the caller to handle

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    for line in results {
        println!("{}", line);
    }
    return Ok(());
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    // pub fn new(args: &Vec<String>) -> Result<Config, &str> { // we'd like this method accepts an iterator to save clone
    //     // constructor
    //     if args.len() < 3 {
    //         return Err("not enough arguments");
    //     }
    //
    //     let query = args[1].clone();
    //     let filename = args[2].clone();
    //     let case_sensitive = env::var("CASE_INSENSITIVE").is_err(); // if CASE_INSENSITICE is not set, return true
    //     return Ok(Config {
    //         query,
    //         filename,
    //         case_sensitive,
    //     });
    // }
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        // we'd like this method accepts an iterator to save clone
        // the lifetime of the return str should be static since we only return string literals
        // constructor
        args.next(); // the first arg is the executable name

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err(); // if CASE_INSENSITICE is not set, return true

        return Ok(Config {
            query,
            filename,
            case_sensitive,
        });
    }
}

/// Search a string in all lines of the contents
///
/// # Examples
/// ```
/// let query = "duct";
/// let contents = "\
/// Rust:
/// safe, fast, productive.
/// Pick three.
/// Duct tape.";
/// let results = minigrep::search(query, contents);
/// assert_eq!(results, vec!["safe, fast, productive."]);
/// ```
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // since the returned &str is borrowed from contents, they should have the same lifetime
    /* Ref from https://doc.rust-lang.org/book/ch12-04-testing-the-librarys-functionality.html
    In other words, we tell Rust that the data returned by the search function will live as long
    as the data passed into the search function in the contents argument. This is important!
    The data referenced by a slice needs to be valid for the reference to be valid; if the compiler
    assumes we’re making string slices of query rather than contents, it will do its safety checking incorrectly.
     */
    // let mut results = Vec::new();
    // for line in contents.lines() {
    //     if line.contains(query) {
    //         results.push(line);
    //     }
    // }
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // since the returned &str is borrowed from contents, they should have the same lifetime
    /* Ref from https://doc.rust-lang.org/book/ch12-04-testing-the-librarys-functionality.html
    In other words, we tell Rust that the data returned by the search function will live as long
    as the data passed into the search function in the contents argument. This is important!
    The data referenced by a slice needs to be valid for the reference to be valid; if the compiler
    assumes we’re making string slices of query rather than contents, it will do its safety checking incorrectly.
     */
    let query = query.to_lowercase(); // old &str query is moved

    // for line in contents.lines() {
    //     if line.to_lowercase().contains(&query) {
    //         results.push(line);
    //     }
    // }
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
    // return results;
}
