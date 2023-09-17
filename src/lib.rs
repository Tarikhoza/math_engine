#![warn(clippy::all, clippy::unwrap_used, clippy::expect_used, clippy::style)]
#![allow(clippy::todo, unused)]

#[macro_use]
extern crate lazy_static;
extern crate fancy_regex;
extern crate regex;
extern crate rust_decimal;

pub mod castable;
pub mod math;
pub mod parser;
pub mod solver;
