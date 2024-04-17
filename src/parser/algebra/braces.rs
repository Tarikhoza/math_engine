use crate::logging::{env_error, env_info, env_warn};

use crate::castable::Castable;
use crate::lexer::{Token, TokenType, Tokenisable};
use crate::math::algebra::braces::Braces;
use crate::math::algebra::polynom::PolynomPart;
use crate::math::Math;
use crate::parser::{Parsable, ParsablePrimitiveAsVariable, ParsableStream, Parser};

impl Parsable for Braces {
    fn to_tex(&self) -> String {
        match &self.exponent {
            Some(has_exp) => match &has_exp.to_tex()[..] {
                "1.0" | "1" => format!("({})", &self.inner.to_tex()),
                exp => format!("({})^{{{}}}", &self.inner.to_tex(), exp),
            },
            _no_exp => format!("({})", &self.inner.to_tex()),
        }
    }

    fn from_token_stream(token_stream: Vec<Token>) -> Result<(PolynomPart, usize), String> {
        env_info("parser", "Trying to parse token stream as brace".into());

        if let Some(inner) =
            Parser::extract_between(&token_stream, TokenType::BraceOpen, TokenType::BraceClose)
        {
            env_info(
                "parser",
                format!("Braces - parsed brace as {:#?}", inner.clone()),
            );
            let mut len = inner.len() + 2;
            let mut exponent: Option<Box<Math>> = None;

            if let Some(exponent_tokens) = token_stream.get(len..) {
                if let Some(ext_exp_tokens) = Parser::extract_exponent(exponent_tokens.to_vec()) {
                    len += ext_exp_tokens.len() + 3;
                    exponent = Some(Box::new(Parser::new(ext_exp_tokens).parse()?.0));
                }
            }

            return Ok((
                PolynomPart::Math(Math::Braces(Braces {
                    inner: Box::new(Parser::new(inner).parse()?.0),
                    exponent,
                })),
                len,
            ));
        } else {
            Err("Failed parsing absolute".into())
        }
    }
}
