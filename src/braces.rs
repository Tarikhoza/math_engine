#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::perf,
    clippy::cargo
)]

use std::ops;

use fancy_regex::Regex;
use rust_decimal_macros::dec;

use crate::math::Math;
use crate::parser::{Parsable, Parser};
//use crate::polynom::Polynom;
use crate::variable::Variable;

#[derive(Debug, Clone)]
pub struct Braces {
    pub math: Box<Math>,
    pub exponent: Option<Box<Math>>,
}

impl Braces {
    pub fn addition(&self, other: Math) -> Math {
        self.simplify() + other.simplify()
    }
    pub fn subtraction(&self, other: Math) -> Math {
        self.simplify() - other.simplify()
    }
    pub fn multiplication(&self, other: Math) -> Math {
        self.simplify() * other.simplify()
    }

    pub fn division(&self, other: Math) -> Math {
        self.simplify() / other.simplify()
    }
    pub fn negative(&self) -> Math {
        match &self.exponent {
            Some(_has_exp) => Math::Braces(Braces {
                math: Box::new(self.math.negative()),
                exponent: Some(Box::new(self.get_exponent())),
            }),
            _no_exp => Math::Braces(Braces {
                math: Box::new(self.math.negative()),
                exponent: None,
            }),
        }
    }

    pub fn simplify(&self) -> Math {
        //TODO apply exponent
        self.math.simplify()
    }
}

impl Braces {
    pub fn new(tex: &str) -> Math {
        Braces::from_tex(tex.to_string())
    }
    pub fn get_exponent(&self) -> Math {
        match &self.exponent {
            None => Math::Variable(Variable {
                value: dec!(1.0),
                suffix: "".to_string(),
                exponent: None,
            }),
            Some(e) => *e.clone(),
        }
    }
}

impl Parsable for Braces {
    fn to_tex(&self) -> String {
        match &self.exponent {
            Some(has_exp) => match &has_exp.to_tex()[..] {
                "1.0" | "1" => format!("({})", &self.math.to_tex()),
                exp => format!("({})^{{{}}}", &self.math.to_tex(), exp),
            },
            _no_exp => format!("({})", &self.math.to_tex()),
        }
    }

    fn from_tex(tex: String) -> Math {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?<!_)\((.+)").unwrap();
        }
        let result = RE.captures(&tex);
        let captures = result
            .expect("Error running regex")
            .expect("No match found");
        let math = Parser::extract_brace(
            captures
                .get(0)
                .map(|m| m.as_str())
                .unwrap_or("")
                .to_string(),
            '(',
            ')',
        );
        let exponent_str = tex.split_at(math.len() + 2).1;
        let exponent: Option<Box<Math>>;
        if !exponent_str.is_empty()
            && exponent_str.chars().nth(0) == Some('^')
            && exponent_str.chars().nth(1) == Some('{')
        {
            exponent = Some(Box::new(
                Parser::new(&(Parser::extract_brace(exponent_str[1..].to_string(), '{', '}')))
                    .parse(),
            ));
        } else {
            exponent = None;
        }

        Math::Braces(Braces {
            math: Box::new(Parser::new(&math).parse()),
            exponent,
        })
    }

    fn on_begining(tex: String) -> Option<String> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(?<!_)\((.+)").unwrap();
        }
        let f = RE.find(&tex).unwrap()?.as_str();
        if !f.is_empty() {
            return Some(f.to_string());
        }
        return None;
    }
}

impl ops::Add<Math> for Braces {
    type Output = Math;
    fn add(self, _rhs: Math) -> Math {
        match _rhs {
            Math::Polynom(p) => self.addition(Math::Polynom(p)),
            Math::Variable(v) => self.addition(Math::Variable(v)),
            Math::Braces(b) => self.addition(Math::Braces(b)),
            Math::Undefined(u) => Math::Undefined(u),
            _ => todo!(),
        }
    }
}

impl ops::Sub<Math> for Braces {
    type Output = Math;
    fn sub(self, _rhs: Math) -> Math {
        match _rhs {
            Math::Polynom(p) => self.subtraction(Math::Polynom(p)),
            Math::Variable(v) => self.subtraction(Math::Variable(v)),
            Math::Braces(b) => self.subtraction(Math::Braces(b)),
            Math::Undefined(u) => Math::Undefined(u),
            _ => todo!(),
        }
    }
}

impl ops::Mul<Math> for Braces {
    type Output = Math;
    fn mul(self, _rhs: Math) -> Math {
        match _rhs {
            Math::Polynom(p) => self.multiplication(Math::Polynom(p)),
            Math::Variable(v) => self.multiplication(Math::Variable(v)),
            Math::Braces(b) => self.multiplication(Math::Braces(b)),
            Math::Undefined(u) => Math::Undefined(u),
            _ => todo!(),
        }
    }
}

impl ops::Div<Math> for Braces {
    type Output = Math;
    fn div(self, _rhs: Math) -> Math {
        match _rhs {
            //           Math::Polynom(p)  => self.mul_brace(Math::Polynom(p)),
            //           Math::Variable(v) => self.mul_brace(Math::Variable(v)),
            //           Math::Braces(b)   => self.mul_brace(Math::Braces(b)),
            //           Math::Undefined(u) => Math::Undefined(u),
            _ => todo!(),
        }
    }
}
