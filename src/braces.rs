use std::ops;

use fancy_regex::Regex;
use rust_decimal_macros::dec;

use crate::math::{BasicOperations, Math};
use crate::parser::{Parsable, Parser};
//use crate::polynom::Polynom;
use crate::variable::Variable;

#[derive(Debug, Clone)]
pub struct Braces {
    pub math: Box<Math>,
    pub exponent: Option<Box<Math>>,
}

impl BasicOperations for Braces {
    #[must_use]
    fn addition(&self, other: Braces) -> Math {
        self.simplify() + other.simplify()
    }
    #[must_use]
    fn subtraction(&self, other: Braces) -> Math {
        self.simplify() - other.simplify()
    }
    #[must_use]
    fn multiplication(&self, other: Braces) -> Math {
        self.simplify() * other.simplify()
    }

    #[must_use]
    fn division(&self, other: Braces) -> Math {
        self.simplify() / other.simplify()
    }
    #[must_use]
    fn negative(&self) -> Math {
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

    #[must_use]
    fn simplify(&self) -> Math {
        //TODO apply exponent
        self.math.simplify()
    }
}

impl Braces {
    #[must_use]
    pub fn get_exponent(&self) -> Math {
        match &self.exponent {
            None => Math::Variable(Variable {
                value: dec!(1.0),
                suffix: String::new(),
                exponent: None,
            }),
            Some(e) => *e.clone(),
        }
    }
}

impl Parsable for Braces {
    #[must_use]
    fn to_tex(&self) -> String {
        match &self.exponent {
            Some(has_exp) => match &has_exp.to_tex()[..] {
                "1.0" | "1" => format!("({})", &self.math.to_tex()),
                exp => format!("({})^{{{}}}", &self.math.to_tex(), exp),
            },
            _no_exp => format!("({})", &self.math.to_tex()),
        }
    }

    #[must_use]
    fn from_tex(tex: &str) -> Result<Math, &'static str> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?<!_)\((.+)").unwrap_or_else(|e| {
                panic!("Failed to compile regex for braces: {e}");
            });
        }
        let result = RE.captures(tex);
        let captures = result
            .expect("Error running regex")
            .expect("No match found");
        let math = Parser::extract_brace(&captures.get(0).map_or("", |m| m.as_str()), '(', ')');
        let exponent_str = tex.split_at(math.len() + 2).1;

        let exponent: Option<Box<Math>> = if !exponent_str.is_empty()
            && exponent_str.starts_with('^')
            && exponent_str.chars().nth(1) == Some('{')
        {
            Some(Box::new(
                Parser::new(&(Parser::extract_brace(&exponent_str[1..], '{', '}'))).parse()?,
            ))
        } else {
            None
        };

        Ok(Math::Braces(Braces {
            math: Box::new(Parser::new(&math).parse()?),
            exponent,
        }))
    }

    #[must_use]
    fn on_begining(tex: String) -> Option<String> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(?<!_)\((.+)").unwrap_or_else(|e| {
                panic!("Failed to compile regex for braces: {e}");
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

impl ops::Add<Math> for Braces {
    type Output = Math;
    fn add(self, rhs: Math) -> Math {
        match rhs {
            Math::Polynom(p) => self.simplify() + Math::Polynom(p),
            Math::Variable(v) => self.simplify() + (Math::Variable(v)),
            Math::Braces(b) => self.simplify() + Math::Braces(b),
            Math::Undefined(u) => Math::Undefined(u),
            _ => todo!(),
        }
    }
}

impl ops::Sub<Math> for Braces {
    type Output = Math;
    fn sub(self, rhs: Math) -> Math {
        match rhs {
            Math::Polynom(p) => self.simplify() - Math::Polynom(p),
            Math::Variable(v) => self.simplify() - Math::Variable(v),
            Math::Braces(b) => self.simplify() - Math::Braces(b).simplify(),
            Math::Undefined(u) => Math::Undefined(u),
            _ => todo!(),
        }
    }
}

impl ops::Mul<Math> for Braces {
    type Output = Math;
    fn mul(self, rhs: Math) -> Math {
        match rhs {
            Math::Polynom(p) => self.simplify() * Math::Polynom(p),
            Math::Variable(v) => self.simplify() * Math::Variable(v),
            Math::Braces(b) => self.simplify() * Math::Braces(b),
            Math::Undefined(u) => Math::Undefined(u),
            _ => todo!(),
        }
    }
}

impl ops::Div<Math> for Braces {
    type Output = Math;
    fn div(self, _rhs: Math) -> Math {
        todo!()
    }
}
