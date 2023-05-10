#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::unwrap_used,
    clippy::style,
    clippy::panicking_unwrap
)]
#![allow(clippy::todo)]
#![allow(unused)]
#![allow(clippy::must_use_candidate)]

#[macro_use]
extern crate lazy_static;
extern crate fancy_regex;
extern crate regex;
extern crate rust_decimal;

pub mod math;
pub mod parser;
pub mod solver;
