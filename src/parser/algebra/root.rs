use crate::logging::{env_error, env_info, env_warn};

use crate::castable::Castable;
use crate::lexer::{Token, TokenType, Tokenisable};
use crate::math::algebra::fraction::Fraction;
use crate::math::algebra::polynom::PolynomPart;
use crate::math::algebra::root::Root;
use crate::math::Math;
use crate::parser::{Parsable, ParsablePrimitiveAsVariable, ParsableStream, Parser};

impl Parsable for Root {
    fn to_tex(&self) -> String {
        if let Some(base) = self.base.clone() {
            format!("\\sqrt[{}]{{{}}}", base.to_tex(), self.inner.to_tex())
        } else {
            format!("\\sqrt[]{{{}}}", self.inner.to_tex())
        }
    }

    fn from_token_stream(token_stream: Vec<Token>) -> Result<(PolynomPart, usize), String> {
        env_info("parser", "Trying to parse token stream as root".into());
        let mut len = 1;
        let base: Option<Box<Math>>;
        let math: Vec<Token>;

        env_info("parser", "Looking for SqrtStart token".into());
        if let Some(token) = token_stream.first() {
            if token.token_type != TokenType::SqrtStart {
                return Err("No sqrt token on begining".into());
            }
        } else {
            return Err("No sqrt token on begining".into());
        }

        env_info("parser", "Extracting base...".into());
        if let Some(base_stream) = token_stream.get(1..) {
            if let Some(base_inner) = Parser::extract_between(
                &base_stream.to_vec(),
                TokenType::SquareOpen,
                TokenType::SquareClose,
            ) {
                len += base_inner.len() + 2;
                base = Some(Box::new(Parser::new(base_inner).parse()?.0));
            } else {
                base = None;
            }
        } else {
            return Err("Failed extracting base".into());
        }

        env_info("parser", "Extracting inner math...".into());
        if let Some(math_stream) = token_stream.get(len..) {
            if let Some(math_inner) = Parser::extract_between(
                &math_stream.to_vec(),
                TokenType::CurlyOpen,
                TokenType::CurlyClose,
            ) {
                math = math_inner;
                len += math.len() + 2;
            } else {
                return Err("Failed extracting math".into());
            }
        } else {
            return Err("Failed extracting math".into());
        }

        return Ok((
            PolynomPart::Math(
                Root {
                    base,
                    inner: Box::new(Parser::new(math).parse()?.0),
                }
                .as_math(),
            ),
            len,
        ));
    }
}
