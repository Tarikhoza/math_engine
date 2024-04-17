#![warn(clippy::all, clippy::unwrap_used, clippy::expect_used, clippy::style)]
#![allow(clippy::todo, unused)]

#[macro_use]
extern crate rust_decimal;

pub mod castable;
pub mod definitions;
pub mod file;
pub mod lexer;
pub mod logging;
pub mod math;
pub mod parser;
pub mod scope;
