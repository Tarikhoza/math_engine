use crate::math::AlgebraOperations;
use crate::math::Math;

#[derive(Debug, Clone, PartialEq)]
pub struct Absolute {
    pub math: Box<Math>,
}

impl Absolute {
    pub fn simplify(&self) -> Math {
        match self.math.simplify() {
            Math::Variable(v) => {
                if v.value.is_sign_negative() {
                    v.negative()
                } else {
                    Math::Variable(v)
                }
            }
            _ => todo!(),
        }
    }
}
