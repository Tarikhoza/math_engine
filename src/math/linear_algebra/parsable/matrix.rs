use crate::math::linear_algebra::matrix::Matrix;
use crate::math::Math;
use crate::parser::Parsable;

impl Parsable for Matrix {
    fn to_tex(&self) -> String {
        let s: String = self
            .factors
            .iter()
            .map(|m| m.to_tex())
            .collect::<Vec<_>>()
            .join(",\n ");

        format!("[\n {}\n]", s)
    }

    fn from_tex(tex: &str) -> Result<Math, &'static str> {
        todo!()
    }

    fn on_begining(_tex: String) -> Option<String> {
        None
    }
}
