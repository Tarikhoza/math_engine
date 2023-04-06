use std::ops;

use fancy_regex::Regex;

use crate::braces::Braces;
use crate::parser::{Parsable, Parser};
use crate::polynom::Polynom;
use crate::undefined::Undefined;
use crate::variable::Variable;

#[derive(Debug, Clone)]
pub enum Math {
    Variable(Variable),
    Polynom(Polynom),
    Braces(Braces),
    Undefined(Undefined),
    Operators(Operators),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operators {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    InvMulti,
}

impl Parsable for Operators {
    #[must_use]
    fn to_tex(&self) -> String {
        match self {
            Operators::Addition => "+".to_owned(),
            Operators::Subtraction => "-".to_owned(),
            Operators::Multiplication => "*".to_owned(),
            Operators::Division => "/".to_owned(),
            Operators::InvMulti => String::new(),
            _ => panic!("Conversion from operator to string went wrong"),
        }
    }

    fn from_tex(op: &str) -> Result<Math, &'static str> {
        match op {
            x if x == "+" => Ok(Math::Operators(Operators::Addition)),
            x if x == "-" => Ok(Math::Operators(Operators::Subtraction)),
            x if x == "*" => Ok(Math::Operators(Operators::Multiplication)),
            x if x == "/" => Ok(Math::Operators(Operators::Division)),
            x if x == " " => Ok(Math::Operators(Operators::InvMulti)),
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

pub trait BasicOperations {
    fn addition(&self, other: Self) -> Math;
    fn subtraction(&self, other: Self) -> Math;
    fn division(&self, other: Self) -> Math;
    fn multiplication(&self, other: Self) -> Math;
    fn simplify(&self) -> Math;
    fn negative(&self) -> Math;
}

impl Math {
    #[must_use]
    pub fn simplify(&self) -> Math {
        match self {
            Math::Polynom(p) => p.simplify(),
            Math::Braces(b) => b.simplify(),
            s => s.clone(),
        }
    }

    #[must_use]
    pub fn sort_score(&self) -> u32 {
        match self {
            Math::Variable(v) => v.sort_score(),
            _ => u32::MAX,
        }
    }

    #[must_use]
    pub fn negative(&self) -> Math {
        match self {
            Math::Variable(s) => s.negative(),
            Math::Polynom(s) => s.negative(),
            Math::Braces(s) => s.negative(),
            Math::Braces(s) => s.negative(),
            s => s.clone(),
        }
    }

    #[must_use]
    pub fn split_operator(&self) -> (Operators, Math) {
        match self {
            Math::Variable(s) => s.split_operator(),
            _ => (Operators::Addition, self.clone()),
        }
    }

    #[must_use]
    pub fn operators_to_string(op: &Operators) -> String {
        op.to_tex()
    }

    #[must_use]
    pub fn string_to_operators(op: String) -> Operators {
        match op {
            x if x == *"+" => Operators::Addition,
            x if x == *"-" => Operators::Subtraction,
            x if x == *"*" => Operators::Multiplication,
            x if x == *"/" => Operators::Division,
            x if x == *" " => Operators::InvMulti,
            String { .. } => todo!(),
        }
    }

    #[must_use]
    pub fn add_sub_change(&self, other: &Math) -> bool {
        let before = format!("{}+{}", self.to_tex(), other.to_tex());
        let after = (self.clone() + other.clone()).to_tex();
        after != before
    }
}

impl Parsable for Math {
    #[must_use]
    fn to_tex(&self) -> String {
        match self {
            Math::Variable(s) => s.to_tex(),
            Math::Braces(s) => s.to_tex(),
            Math::Polynom(s) => s.to_tex(),
            Math::Undefined(s) => s.to_tex(),
            Math::Operators(s) => s.to_tex(),
            _ => todo!(),
        }
    }

    fn from_tex(tex: &str) -> Result<Math, &'static str> {
        Parser::new(tex).parse()
    }

    #[must_use]
    fn on_begining(_tex: String) -> Option<String> {
        None
    }
}

impl ops::Add<Math> for Math {
    type Output = Math;
    fn add(self, rhs: Math) -> Math {
        match self {
            Math::Polynom(p) => p + rhs,
            Math::Variable(v) => v + rhs,
            Math::Braces(b) => b + rhs,
            Math::Undefined(u) => Math::Undefined(u),
            _ => todo!(),
        }
    }
}

impl ops::Sub<Math> for Math {
    type Output = Math;
    fn sub(self, rhs: Math) -> Math {
        match self {
            Math::Polynom(p) => p - rhs,
            Math::Variable(v) => v - rhs,
            Math::Braces(b) => b - rhs,
            Math::Undefined(u) => Math::Undefined(u),
            _ => todo!(),
        }
    }
}

impl ops::Mul<Math> for Math {
    type Output = Math;
    fn mul(self, rhs: Math) -> Math {
        match self {
            Math::Polynom(p) => p * rhs,
            Math::Variable(v) => v * rhs,
            Math::Braces(b) => b * rhs,
            Math::Undefined(u) => Math::Undefined(u),
            Math::Operators(_) => todo!(),
        }
    }
}

impl ops::Div<Math> for Math {
    type Output = Math;
    fn div(self, rhs: Math) -> Math {
        match self {
            //   Math::Polynom(p)  => p*_rhs,
            //   Math::Braces(b)   => b*_rhs,
            Math::Variable(v) => v / rhs,
            Math::Undefined(u) => Math::Undefined(u),
            _ => todo!(),
        }
    }
}
