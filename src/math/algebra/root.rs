use crate::math::algebra::braces::Braces;
use crate::math::Math;
//use crate::variable::Variable;
//use std::ops;

#[derive(Debug, Clone)]
pub struct Root {
    pub math: Math,
    pub base: Box<Math>,
}

impl Root {
    #[must_use]
    pub fn to_tex(&self) -> String {
        todo!();
    }

    #[must_use]
    pub fn negative(&self) -> Braces {
        todo!();
    }
}
