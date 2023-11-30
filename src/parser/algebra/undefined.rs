use crate::logging::{env_error, env_info, env_warn};

use crate::lexer::{Token, TokenType, Tokenisable};
use crate::math::algebra::polynom::PolynomPart;
use crate::math::algebra::undefined::Undefined;
use crate::math::Math;
use crate::parser::{Parsable, ParsablePrimitiveAsVariable, Parser};

impl Parsable for Undefined {
    fn to_tex(&self) -> String {
        return String::from("undefined");
    }

    fn from_token_stream(token_stream: Vec<Token>) -> Result<(PolynomPart, usize), String> {
        todo!()
    }
}
