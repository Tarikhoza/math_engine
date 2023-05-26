use crate::math::Math;

#[derive(Debug, Clone)]
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
}
