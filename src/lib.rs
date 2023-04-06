#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::perf,
    clippy::cargo
)]
#[macro_use]
extern crate lazy_static;
extern crate fancy_regex;
extern crate rust_decimal;

pub mod braces;
pub mod math;
pub mod parser;
pub mod polynom;
pub mod root;
pub mod undefined;
pub mod variable;
