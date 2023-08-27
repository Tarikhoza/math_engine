pub mod braces;
pub mod fraction;
pub mod infinity;
pub mod polynom;
pub mod root;
pub mod undefined;
pub mod variable;

use crate::math::simplifiable::Simplifiable;
use crate::math::Math;

use crate::math::operator::Operator as MainOperator;
use crate::parser::{Parsable, Parser};
use fancy_regex::Regex;

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    InvMulti,
    AddSub,
}

pub trait Operations: Simplifiable {
    fn add_self(&self, other: &Self) -> Math;
    fn sub_self(&self, other: &Self) -> Math;
    fn div_self(&self, other: &Self) -> Math;
    fn mul_self(&self, other: &Self) -> Math;

    fn negative(&self) -> Math;
    fn substitute(&self, suffix: &str, value: Math) -> Math;

    fn add(&self, other: &Math) -> Math;
    fn sub(&self, other: &Math) -> Math;
    fn div(&self, other: &Math) -> Math;
    fn mul(&self, other: &Math) -> Math;
    fn add_sub(&self, other: &Math) -> (Math, Math) {
        (self.add(other), self.sub(other))
    }

    fn get_all_suffixes(&self) -> Vec<String>;
}

impl Operator {
    pub fn to_tex(&self) -> String {
        match self {
            Operator::Addition => "+".to_owned(),
            Operator::Subtraction => "-".to_owned(),
            Operator::Multiplication => "*".to_owned(),
            Operator::Division => "/".to_owned(),
            Operator::InvMulti => String::new(),
            Operator::AddSub => "\\pm".to_owned(),
            _ => panic!("Conversion from operator to string went wrong"),
        }
    }

    pub fn from_tex(op: &str) -> Result<Operator, &'static str> {
        match op {
            x if x == "+" => Ok(Operator::Addition),
            x if x == "-" => Ok(Operator::Subtraction),
            x if x == "*" => Ok(Operator::Multiplication),
            x if x == "/" => Ok(Operator::Division),
            x if x == "" => Ok(Operator::InvMulti),
            x if x == "\\pm" => Ok(Operator::AddSub),
            _ => Err("Conversion from string to operator went wrong"),
        }
    }

    pub fn on_begining(tex: String) -> Option<String> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^[\-+\/*]|\\pm").unwrap_or_else(|e| {
                panic!("Failed to compile regex for operators: {e}");
            });
        }
        if let Ok(Some(f)) = RE.find(&tex) {
            let f_str = f.as_str().to_string();
            if !f_str.is_empty() {
                return Some(f_str);
            }
        }
        None
    }
}
