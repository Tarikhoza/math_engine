#![warn(
    clippy::all,
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

pub mod castable;
pub mod math;
pub mod parser;
pub mod solver;
