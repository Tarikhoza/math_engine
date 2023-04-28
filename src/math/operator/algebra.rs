use crate::math::operator::Operator as MainOperator;
use crate::math::Math;
use crate::parser::{Parsable, Parser};
use fancy_regex::Regex;

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    InvMulti,
}

pub trait Operations {
    fn addition(&self, other: &Self) -> Math;
    fn subtraction(&self, other: &Self) -> Math;
    fn division(&self, other: &Self) -> Math;
    fn multiplication(&self, other: &Self) -> Math;

    fn simplify(&self) -> Math;
    fn negative(&self) -> Math;

    fn add(&self, other: &Math) -> Math;
    fn sub(&self, other: &Math) -> Math;
    fn div(&self, other: &Math) -> Math;
    fn mul(&self, other: &Math) -> Math;
}

impl Parsable for Operator {
    #[must_use]
    fn to_tex(&self) -> String {
        match self {
            Operator::Addition => "+".to_owned(),
            Operator::Subtraction => "-".to_owned(),
            Operator::Multiplication => "*".to_owned(),
            Operator::Division => "/".to_owned(),
            Operator::InvMulti => String::new(),
            _ => panic!("Conversion from operator to string went wrong"),
        }
    }

    fn from_tex(op: &str) -> Result<Math, &'static str> {
        match op {
            x if x == "+" => Ok(Math::Operator(MainOperator::Algebra(Operator::Addition))),
            x if x == "-" => Ok(Math::Operator(MainOperator::Algebra(Operator::Subtraction))),
            x if x == "*" => Ok(Math::Operator(MainOperator::Algebra(
                Operator::Multiplication,
            ))),
            x if x == "/" => Ok(Math::Operator(MainOperator::Algebra(Operator::Division))),
            x if x == String::new()=> Ok(Math::Operator(MainOperator::Algebra(Operator::InvMulti))),
            _ => Err("Conversion from string to operator went wrong"),
        }
    }
    #[must_use]
    fn on_begining(tex: String) -> Option<String> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^[\-+\/*]").unwrap_or_else(|e| {
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
