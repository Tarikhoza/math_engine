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
    fn to_tex(&self) -> String {
        match self {
            Operators::Addition => "+".to_string(),
            Operators::Subtraction => "-".to_string(),
            Operators::Multiplication => "*".to_string(),
            Operators::Division => "/".to_string(),
            Operators::InvMulti => "".to_string(),
            _ => panic!("Conversion from operator to string went wrong"),
        }
    }
    fn from_tex(op: String) -> Math {
        match op {
            x if x == "+".to_string() => Math::Operators(Operators::Addition),
            x if x == "-".to_string() => Math::Operators(Operators::Subtraction),
            x if x == "*".to_string() => Math::Operators(Operators::Multiplication),
            x if x == "/".to_string() => Math::Operators(Operators::Division),
            x if x == " ".to_string() => Math::Operators(Operators::InvMulti),
            _ => panic!("Conversion from string to operator went wrong"),
        }
    }
    fn on_begining(tex: String) -> Option<String> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^[\-+\/*]").unwrap();
        }
        let f = RE.find(&tex).unwrap()?.as_str();
        if !f.is_empty() {
            return Some(f.to_string());
        }
        return None;
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
    pub fn simplify(&self) -> Math {
        match self {
            Math::Polynom(p) => p.simplify(),
            Math::Braces(b) => b.simplify(),
            s => s.clone(),
        }
    }

    pub fn sort_score(&self) -> u32 {
        match self {
            Math::Variable(v) => v.sort_score(),
            _ => u32::MAX,
        }
    }

    pub fn negative(&self) -> Math {
        match self {
            Math::Variable(s) => s.negative(),
            Math::Polynom(s) => s.negative(),
            Math::Braces(s) => s.negative(),
            s => s.clone(),
        }
    }

    pub fn split_operator(&self) -> (Operators, Math) {
        match self {
            Math::Variable(s) => s.split_operator(),
            _ => (Operators::Addition, self.clone()),
        }
    }

    pub fn operators_to_string(op: Operators) -> String {
        match op {
            Operators::Addition => "+".to_string(),
            Operators::Subtraction => "-".to_string(),
            Operators::Multiplication => "*".to_string(),
            Operators::Division => "/".to_string(),
            Operators::InvMulti => "".to_string(),
            _ => panic!("Conversion from operator to string went wrong"),
        }
    }
    pub fn string_to_operators(op: String) -> Operators {
        match op {
            x if x == "+".to_string() => Operators::Addition,
            x if x == "-".to_string() => Operators::Subtraction,
            x if x == "*".to_string() => Operators::Multiplication,
            x if x == "/".to_string() => Operators::Division,
            x if x == " ".to_string() => Operators::InvMulti,
            _ => panic!("Conversion from string to operator went wrong"),
        }
    }

    pub fn add_sub_change(&self, other: &Math) -> bool {
        let before = format!("{}+{}", self.to_tex(), other.to_tex());
        let after = (self.clone() + other.clone()).to_tex();
        after != before
    }
}

impl Parsable for Math {
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

    fn from_tex(tex: String) -> Math {
        Parser::new(&tex).parse()
    }

    fn on_begining(tex: String) -> Option<String> {
        None
    }
}

impl ops::Add<Math> for Math {
    type Output = Math;
    fn add(self, _rhs: Math) -> Math {
        match self {
            Math::Polynom(p) => p + _rhs,
            Math::Variable(v) => v + _rhs,
            Math::Braces(b) => b + _rhs,
            Math::Undefined(u) => Math::Undefined(u),
            _ => todo!(),
        }
    }
}

impl ops::Sub<Math> for Math {
    type Output = Math;
    fn sub(self, _rhs: Math) -> Math {
        match self {
            Math::Polynom(p) => p - _rhs,
            Math::Variable(v) => v - _rhs,
            Math::Braces(b) => b - _rhs,
            Math::Undefined(u) => Math::Undefined(u),
            _ => todo!(),
        }
    }
}

impl ops::Mul<Math> for Math {
    type Output = Math;
    fn mul(self, _rhs: Math) -> Math {
        match self {
            Math::Polynom(p) => p * _rhs,
            Math::Variable(v) => v * _rhs,
            Math::Braces(b) => b * _rhs,
            Math::Undefined(u) => Math::Undefined(u),
            _ => todo!(),
        }
    }
}

impl ops::Div<Math> for Math {
    type Output = Math;
    fn div(self, _rhs: Math) -> Math {
        match self {
            //   Math::Polynom(p)  => p*_rhs,
            //   Math::Braces(b)   => b*_rhs,
            Math::Variable(v) => v / _rhs,
            Math::Undefined(u) => Math::Undefined(u),
            _ => todo!(),
        }
    }
}
