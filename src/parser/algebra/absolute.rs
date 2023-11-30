use crate::logging::{env_error, env_info, env_warn};

use crate::castable::Castable;
use crate::lexer::{Token, TokenType, Tokenisable};
use crate::math::algebra::absolute::Absolute;
use crate::math::algebra::polynom::PolynomPart;
use crate::math::Math;
use crate::parser::{Parsable, ParsablePrimitiveAsVariable, ParsableStream, Parser};

impl Parsable for Absolute {
    fn to_tex(&self) -> String {
        format!("\\lvert{}\\rvert", self.inner.to_tex())
    }
    fn from_token_stream(token_stream: Vec<Token>) -> Result<(PolynomPart, usize), String> {
        env_info("parser", "Trying to parse token stream as absolute".into());

        if let Some(inner) =
            Parser::extract_between(&token_stream, TokenType::VertOpen, TokenType::VertClose)
        {
            env_info(
                "parser",
                format!("Absolute - parsed absolute as {:#?}", inner.clone()),
            );
            let len = inner.len() + 2;

            return Ok((
                PolynomPart::Math(Math::Absolute(Absolute {
                    inner: Box::new(Parser::new(inner).parse()?),
                })),
                len,
            ));
        } else {
            Err("Failed parsing absolute".into())
        }
    }
}
