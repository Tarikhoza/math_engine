use crate::logging::{env_error, env_info, env_warn};

use crate::lexer::{Token, TokenType, Tokenisable};
use crate::math::algebra::infinity::Infinity;
use crate::math::algebra::polynom::PolynomPart;
use crate::math::algebra::undefined::Undefined;
use crate::math::Math;
use crate::parser::{Parsable, ParsablePrimitiveAsVariable, Parser};

impl Parsable for Infinity {
    fn to_tex(&self) -> String {
        String::from("\\infty")
    }

    fn from_token_stream(token_stream: Vec<Token>) -> Result<(PolynomPart, usize), String> {
        todo!()
    }
}
