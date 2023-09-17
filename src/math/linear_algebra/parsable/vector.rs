use crate::math::linear_algebra::vector::Vector;
use crate::math::Math;
use crate::parser::Parsable;

impl Parsable for Vector {
    fn to_tex(&self) -> String {
        let s: String = self
            .factors
            .iter()
            .map(|m| m.to_tex())
            .collect::<Vec<_>>()
            .join(",\t");

        format!("[{}]", s)
    }

    fn from_tex(_tex: &str) -> Result<Math, &'static str> {
        todo!()
    }

    fn on_begining(_tex: String) -> Option<String> {
        None
    }

    fn from_tex_len(_tex: &str) -> Result<(usize, Math), &'static str> {
        todo!()
    }
}
