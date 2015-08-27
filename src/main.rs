#![feature(core_intrinsics)]
#![allow(dead_code)]
#![allow(unused_variables)]
extern crate regex;

mod binding;
mod matching;
mod error_handling;
mod ownership;

fn main() {
    binding::binding();
    //matching::matching();
    //error_handling::errors();
    //ownership::ownership();
}
