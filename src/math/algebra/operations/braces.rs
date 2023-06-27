use crate::math::algebra::braces::Braces;
use crate::math::algebra::exponentable::Exponentable;
//use crate::math::algebra::undefined::Undefined;
use crate::math::algebra::variable::Variable;
use crate::math::operator::algebra::{Operations, Operator};
use std::ops;

use crate::math::Math;
use rust_decimal_macros::dec;

impl Operations for Braces {
    fn addition(&self, other: &Braces) -> Math {
        self.simplify().add(&other.simplify())
    }
    fn subtraction(&self, other: &Braces) -> Math {
        self.simplify().sub(&other.simplify())
    }
    fn multiplication(&self, other: &Braces) -> Math {
        self.simplify().mul(&other.simplify())
    }

    fn division(&self, other: &Braces) -> Math {
        self.simplify().div(&other.simplify())
    }
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

    fn simplify(&self) -> Math {
        self.apply_exponent()
    }

    fn add(&self, rhs: &Math) -> Math {
        match rhs {
            Math::Polynom(p) => self.simplify().addition(&Math::Polynom(p.clone())),
            Math::Variable(v) => self.simplify().addition(&Math::Variable(v.clone())),
            Math::Braces(b) => self
                .simplify()
                .addition(&Math::Braces(b.clone()).simplify()),
            //          Math::Undefined(u) => Math::Undefined(Undefined {}),
            _ => todo!(),
        }
    }

    fn sub(&self, rhs: &Math) -> Math {
        match rhs {
            Math::Polynom(p) => self.simplify().addition(&Math::Polynom(p.clone())),
            Math::Variable(v) => self.simplify().addition(&Math::Variable(v.clone())),
            Math::Braces(b) => self
                .simplify()
                .addition(&Math::Braces(b.clone()).simplify()),
            //           Math::Undefined(u) => Math::Undefined(Undefined {}),
            _ => todo!(),
        }
    }

    fn mul(&self, rhs: &Math) -> Math {
        match rhs {
            Math::Polynom(p) => self.simplify().addition(&Math::Polynom(p.clone())),
            Math::Variable(v) => self.simplify().addition(&Math::Variable(v.clone())),
            Math::Braces(b) => self
                .simplify()
                .addition(&Math::Braces(b.clone()).simplify()),
            //            Math::Undefined(u) => Math::Undefined(Undefined {}),
            _ => todo!(),
        }
    }

    fn div(&self, _rhs: &Math) -> Math {
        todo!()
    }

    fn substitute(&self, suffix: &str, math: Math) -> Math {
        let new_math = Box::new(self.math.substitute(suffix, math.clone()));
        let new_exponent = self.get_exponent().substitute(suffix, math);

        Math::Braces(Braces {
            math: new_math,
            exponent: Some(Box::new(new_exponent)),
        })
    }
    fn get_all_suffixes(&self) -> Vec<String> {
        let mut suf: Vec<String> = vec![];
        suf.extend_from_slice(&self.math.get_all_suffixes());
        suf.extend_from_slice(&self.get_exponent().get_all_suffixes());

        suf.sort();
        suf.dedup();
        suf
    }
}
