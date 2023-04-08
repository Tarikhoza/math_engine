use crate::math::{BasicOperations, Math};
use rust_decimal_macros::dec;
use std::ops;
//use crate::polynom::Polynom;
use crate::variable::Variable;

#[derive(Debug, Clone)]
pub struct Braces {
    pub math: Box<Math>,
    pub exponent: Option<Box<Math>>,
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
