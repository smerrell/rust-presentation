#![feature(core_intrinsics)]
#![allow(dead_code)]
#![allow(unused_variables)]
extern crate regex;

mod binding;
mod error_handling;
mod functions;
mod matching;
mod ownership;
mod borrowing;
mod lifetimes;
mod utils;

fn main() {
    binding::binding();
    //matching::matching();
    //error_handling::errors();
    //functions::functions();
    //ownership::ownership();
    //borrowing::borrowing();
    //lifetimes::lifetimes();
}
