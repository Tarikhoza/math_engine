use crate::math::operator::algebra::Operations;
use crate::math::Math;

#[derive(Debug, Clone, PartialEq)]
pub struct Fraction {
    pub numerator: Box<Math>,
    pub denominator: Box<Math>,
}

impl Fraction {
    pub fn inverse(&self) -> Fraction {
        Fraction {
            numerator: self.denominator.clone(),
            denominator: self.numerator.clone(),
        }
    }
    pub fn expand(&self, other: Math) -> Fraction {
        Fraction {
            numerator: Box::new(self.numerator.mul(&other)),
            denominator: Box::new(self.denominator.mul(&other)),
        }
    }
}
