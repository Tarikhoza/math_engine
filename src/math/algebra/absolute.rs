use crate::math::AlgebraOperations;
use crate::math::Math;

#[derive(Debug, Clone, PartialEq)]
pub struct Absolute {
    pub math: Box<Math>,
}

impl Absolute {
    pub fn simplify(&self) -> Math {
        let math = match self.math.simplify() {
            Math::Variable(v) => {
                if v.value.is_sign_negative() {
                    return v.negative();
                } else {
                    return Math::Variable(v.clone());
                }
            }
            _ => todo!(),
        };
    }
}
