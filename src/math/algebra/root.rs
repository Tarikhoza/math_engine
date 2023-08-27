use rust_decimal::prelude::*;
use rust_decimal_macros::dec;
use std::default;

use crate::math::algebra::braces::Braces;
use crate::math::algebra::exponentable::Exponentable;
use crate::math::algebra::fraction::Fraction;
use crate::math::algebra::variable::Variable;
use crate::math::simplifiable::Simplifiable;
use crate::math::AlgebraOperations;
use crate::math::Math;
use crate::parser::{Parsable, ParsableGenerics, ParsableGenericsAsVariable};
//use crate::variable::Variable;
//use std::ops;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Root {
    pub math: Box<Math>,
    pub base: Option<Box<Math>>,
}

impl Root {
    pub fn square_root(&self) -> Math {
        let mut math = self.math.simplify();
        let mut guess = math.clone();
        let mut result = math.clone();
        let mut last_result = Math::default();
        //TODO implement division in all Math types
        //lazy_static! {
        //    static ref EXPRESSION: Math = "(g+r)/2".parse_math().unwrap();
        //}

        //for i in 1..100 {
        //    result = math.div(&guess);
        //    guess = EXPRESSION
        //        .substitute("g", guess)
        //        .substitute("r", result.clone())
        //        .simplify();
        //    if last_result.to_tex() == result.to_tex() {
        //        break;
        //    }
        //    last_result = result.clone();
        //}
        for i in 1..100 {
            result = math.div(&guess);
            guess = format!("({}+{})/{}", guess.to_tex(), result.to_tex(), 2)
                .parse_math()
                .expect("failed parsing guess as math")
                .simplify();
            if last_result.to_tex() == result.to_tex() {
                break;
            }
            last_result = result.clone();
        }
        result
    }

    pub fn take_root(&self) -> Math {
        if let Some(base) = &self.base {
            if base.to_tex() == "2" {
                return self.square_root();
            } else {
                todo!("implement derivatives to take any root")
            }
        }
        self.square_root()
    }

    pub fn exponential_form(&self) -> Math {
        let exponent: Math = match *self.math.clone() {
            Math::Variable(v) => v.get_exponent(),
            Math::Braces(b) => b.get_exponent(),
            Math::Function(f) => f.get_exponent(),
            _ => "1".parse_math().expect("failed parsing 1 as math"),
        };

        match *self.math.clone() {
            Math::Variable(v) => Math::Variable(Variable {
                value: v.value,
                suffix: v.suffix,
                exponent: Some(Box::new(Math::Fraction(Fraction {
                    whole: None,
                    denominator: Box::new(exponent),
                    numerator: Box::new(self.get_base()),
                }))),
                #[cfg(feature = "step-tracking")]
                step: None,
            }),
            Math::Braces(b) => Math::Braces(Braces {
                math: b.math,
                exponent: Some(Box::new(Math::Fraction(Fraction {
                    whole: None,
                    denominator: Box::new(exponent),
                    numerator: Box::new(self.get_base()),
                }))),
            }),
            other => Math::Braces(Braces {
                math: Box::new(other),
                exponent: Some(Box::new(Math::Fraction(Fraction {
                    whole: None,
                    denominator: Box::new(exponent),
                    numerator: Box::new(self.get_base()),
                }))),
            }),
        }
    }

    pub fn get_base(&self) -> Math {
        if self.base.is_none() {
            "2".parse_math().expect("parsing 2 as math failed")
        } else {
            *self.base.clone().expect("base is none")
        }
    }
}
