#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::unwrap_used,
    clippy::style,
    clippy::panicking_unwrap
)]
#![allow(unreachable_patterns)]
#![allow(clippy::todo)]
#![warn(warnings)]
#![deny(
    bad_style,
    dead_code,
    improper_ctypes,
    non_shorthand_field_patterns,
    no_mangle_generic_items,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    private_in_public,
    unconditional_recursion,
    unused_allocation,
    unused_comparisons,
    unused_parens,
    while_true
)]
#![allow(clippy::todo)]

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
