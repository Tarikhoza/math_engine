use crate::math::simplifiable::Simplifiable;
use crate::math::AlgebraOperations;
use crate::math::Math;

#[derive(Debug, Clone, PartialEq)]
pub struct Absolute {
    pub inner: Box<Math>,
}

impl Simplifiable for Absolute {
    fn simplify(&self) -> Math {
        match self.inner.simplify() {
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

impl AlgebraOperations for Absolute {
    fn add_self(&self, other: &Self) -> Math {
        todo!()
    }

    fn sub_self(&self, other: &Self) -> Math {
        todo!()
    }

    fn div_self(&self, other: &Self) -> Math {
        todo!()
    }

    fn mul_self(&self, other: &Self) -> Math {
        todo!()
    }

    fn substitute(&self, suffix: &str, value: Math) -> Math {
        todo!()
    }

    fn add(&self, other: &Math) -> Math {
        todo!()
    }

    fn sub(&self, other: &Math) -> Math {
        todo!()
    }

    fn div(&self, other: &Math) -> Math {
        todo!()
    }

    fn mul(&self, other: &Math) -> Math {
        todo!()
    }

    fn get_all_suffixes(&self) -> Vec<String> {
        todo!()
    }
}
