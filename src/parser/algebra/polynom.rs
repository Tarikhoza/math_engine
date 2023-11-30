use crate::castable::Castable;
use crate::lexer::{Token, TokenType, Tokenisable};
use crate::logging::{env_error, env_info, env_warn};
use crate::math::algebra::operations::Operator as AlgebraOperator;
use crate::math::algebra::polynom::Polynom;
use crate::math::algebra::variable::Variable;
use crate::math::{algebra::polynom::PolynomPart, Math};
use crate::parser::{Parsable, ParsablePrimitiveAsVariable, Parser};

impl Math {
    pub fn as_polynom_part(&self) -> PolynomPart {
        PolynomPart::Math(self.clone())
    }
}
impl AlgebraOperator {
    pub fn as_polynom_part(&self) -> PolynomPart {
        PolynomPart::Operator(self.clone())
    }
}

impl Parsable for Polynom {
    fn to_tex(&self) -> String {
        let mut tex = String::new();
        for part in self.parts.iter() {
            if let PolynomPart::Math(math) = part {
                tex = format!("{}{}", tex, math.to_tex());
            } else if let PolynomPart::Operator(op) = part {
                tex = format!("{}{}", tex, op.to_tex());
            }
        }
        tex
    }
    fn from_token_stream(token_stream: Vec<Token>) -> Result<(PolynomPart, usize), String> {
        unimplemented!()
    }
}
