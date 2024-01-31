use crate::math::algebra::polynom::PolynomPart;
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

    fn from_token_stream(
        token_stream: Vec<crate::lexer::Token>,
    ) -> Result<(PolynomPart, usize), String> {
        todo!()
    }
}
