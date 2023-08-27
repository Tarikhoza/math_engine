use crate::math::simplifiable::Simplifiable;
use crate::math::AlgebraOperations;
use crate::math::Math;

#[derive(Debug, Clone, PartialEq)]
pub struct Absolute {
    pub math: Box<Math>,
}

impl Simplifiable for Absolute {
    fn simplify(&self) -> Math {
        match self.math.simplify() {
            Math::Variable(v) => {
                if v.is_integer() {
                    if v.value.is_sign_negative() {
                        v.negative()
                    } else {
                        Math::Variable(v)
                    }
                } else {
                    Math::Absolute(self.clone())
                }
            }
            _ => todo!(),
        }
    }
}
