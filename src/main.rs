extern crate regex;

mod matching;
mod error_handling;

fn main() {
    matching::matching();
    error_handling::errors();
}
