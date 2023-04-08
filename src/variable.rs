use std::ops;

use fancy_regex::Regex;
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

use crate::math::{BasicOperations, Math};
use crate::operators::Operators;
use crate::parser::{Parsable, Parser};
use crate::polynom::Polynom;

#[derive(Debug, Clone)]
pub struct Variable {
    pub value: Decimal,
    pub suffix: String,
    pub exponent: Option<Box<Math>>,
}

fn ascii_score(s: &str) -> u32 {
    let mut score = 0;
    for (i, c) in s.chars().enumerate() {
        score = c.to_digit(10).unwrap_or(1000) / (i + 1) as u32;
    }
    score
}

impl Variable {
    #[must_use]
    pub fn get_exponent(&self) -> Math {
        match &self.exponent {
            None => Math::Variable(Variable {
                value: dec!(1),
                suffix: String::new(),
                exponent: None,
            }),
            Some(e) => *e.clone(),
        }
    }

    #[must_use]
    pub fn apply_exponent(&self) -> Math {
        todo!()
    }

    #[must_use]
    pub fn as_polynom(&self) -> Polynom {
        Polynom {
            factors: vec![Math::Variable(self.clone())],
            operators: vec![],
        }
    }

    #[must_use]
    pub fn split_operator(&self) -> (Operators, Math) {
        if self.value < dec!(0) {
            return (Operators::Subtraction, self.negative());
        }
        (Operators::Addition, Math::Variable(self.clone()))
    }

    #[must_use]
    pub fn sort_score(&self) -> u32 {
        u32::MAX - (ascii_score(&self.suffix) + ascii_score(&self.get_exponent().to_tex()))
    }

    #[must_use]
    pub fn add_sub_base(&self) -> String {
        let mut x = self.clone();
        x.value = dec!(1.0);
        x.to_tex()
    }
}

impl ops::Add<Math> for Variable {
    type Output = Math;
    fn add(self, rhs: Math) -> Math {
        //        println!("{}+{}", self.to_tex(), _rhs.to_tex());
        match rhs {
            Math::Polynom(p) => self.as_polynom() + Math::Polynom(p),
            Math::Variable(v) => self.addition(v),
            Math::Undefined(u) => Math::Undefined(u),
            _ => todo!(),
        }
    }
}

impl ops::Sub<Math> for Variable {
    type Output = Math;
    fn sub(self, rhs: Math) -> Math {
        //        println!("{}-{}", self.to_tex(), _rhs.to_tex());
        match rhs {
            Math::Polynom(p) => self.as_polynom() - Math::Polynom(p),
            Math::Variable(v) => self.subtraction(v),
            Math::Undefined(u) => Math::Undefined(u),
            _ => todo!(),
        }
    }
}

impl ops::Mul<Math> for Variable {
    type Output = Math;
    fn mul(self, rhs: Math) -> Math {
        match rhs {
            Math::Variable(v) => self.multiplication(v),
            Math::Polynom(p) => self.as_polynom() * Math::Polynom(p),
            Math::Undefined(u) => Math::Undefined(u),
            _ => todo!(),
        }
    }
}

impl ops::Div<Math> for Variable {
    type Output = Math;
    fn div(self, rhs: Math) -> Math {
        match rhs {
            //  Math::Polynom(p)  => self.as_polynom()*Math::Polynom(p),
            Math::Variable(v) => self.division(v),
            Math::Undefined(u) => Math::Undefined(u),
            _ => todo!(),
        }
    }
}
