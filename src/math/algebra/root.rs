use rust_decimal::prelude::*;
use rust_decimal_macros::dec;
use std::default;

use crate::math::algebra::braces::Braces;
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
                .unwrap()
                .simplify();
            if last_result.to_tex() == result.to_tex() {
                break;
            }
            last_result = result.clone();
        }
        return result;
    }

    pub fn take_root(&self) -> Math {
        if let Some(base) = &self.base {
            if base.to_tex() == "2" {
                return self.square_root();
            } else {
                todo!("implement derivatives to take any root")
            }
        }
        return self.square_root();
    }
}
