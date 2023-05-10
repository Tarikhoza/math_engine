use crate::math::algebra::variable::Variable;
use crate::math::Math;
use rust_decimal_macros::dec;

use crate::solver::step::Step;

#[derive(Debug, Clone)]
pub struct Braces {
    pub math: Box<Math>,
    pub exponent: Option<Box<Math>>,
}

impl Braces {
    pub fn get_exponent(&self) -> Math {
        match &self.exponent {
            None => Math::Variable(Variable {
                value: dec!(1.0),
                suffix: String::new(),
                exponent: None,
                #[cfg(feature = "step-tracking")]
                step:None
            }),
            Some(e) => *e.clone(),
        }
    }
}
