use crate::math::algebra::polynom::PolynomPart;
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

    fn from_token_stream(
        token_stream: Vec<crate::lexer::Token>,
    ) -> Result<(PolynomPart, usize), String> {
        todo!()
    }
}
