use crate::logging::env_info;
use crate::math::algebra::operations::Operations;
use crate::math::linear_algebra::vector::Vector;
use crate::math::Math;

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix {
    pub factors: Vec<Vector>,
}

impl Matrix {
    pub fn add_all(&self) -> Math {
        env_info("helper", format!("add_all before: {:#?}", self.factors));

        let res = self
            .factors
            .iter()
            .map(|v| v.add_all())
            .fold(Math::default(), |acc, e| acc.add(&e));

        env_info("helper", format!("add_all after: {:#?}", res));
        res
    }
}
