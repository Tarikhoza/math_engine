#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::unwrap_used,
    clippy::style,
    clippy::panicking_unwrap
)]
#![allow(clippy::todo)]
#![allow(unused)]

#[macro_use]
extern crate lazy_static;
extern crate fancy_regex;
extern crate regex;
extern crate rust_decimal;

pub mod basic_operations;
pub mod braces;
pub mod equation;
pub mod fraction;
pub mod math;
pub mod matrix;
pub mod operators;
pub mod parsable;
pub mod parser;
pub mod polynom;
pub mod root;
pub mod undefined;
pub mod variable;
pub mod vector;
