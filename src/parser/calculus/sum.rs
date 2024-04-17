use crate::logging::{env_error, env_info, env_warn};

use crate::lexer::{Token, TokenType, Tokenisable};
use crate::math::algebra::infinity::Infinity;
use crate::math::algebra::polynom::PolynomPart;
use crate::math::algebra::undefined::Undefined;
use crate::math::calculus::sum::Sum;
use crate::math::Math;
use crate::parser::{Parsable, ParsablePrimitiveAsVariable, Parser};

impl Parsable for Sum {
    fn to_tex(&self) -> String {
        format!(
            "\\sum_{{{}={}}}^{{{}}}{{{}}}",
            self.iter,
            self.start.to_tex(),
            self.end.to_tex(),
            self.inner.to_tex()
        )
    }

    fn from_token_stream(token_stream: Vec<Token>) -> Result<(PolynomPart, usize), String> {
        env_info("parser", "Trying to parse token stream as sum".into());
        let mut len = 1;
        let iter: String;
        let start: Box<Math>;
        let end: Box<Math>;
        let inner: Box<Math>;

        env_info("parser", "Looking for SumStart token".into());
        if let Some(token) = token_stream.first() {
            if token.token_type != TokenType::SumStart {
                env_info("parser", "No sum token on begining...".into());
                return Err("No sum token on begining".into());
            }
        } else {
            env_info("parser", "No sum token on begining...".into());
            return Err("No sum token on begining".into());
        }

        env_info("parser", "Extracting iter and start...".into());
        if let Some(iter_stream) = token_stream.get(1..) {
            if let Some(iter_inner) = Parser::extract_between(
                &iter_stream.to_vec(),
                TokenType::CurlyOpen,
                TokenType::CurlyClose,
            ) {
                let inner_str = iter_inner
                    .iter()
                    .map(|token| token.literal.clone().unwrap_or_default())
                    .collect::<Vec<String>>()
                    .concat();

                len += inner_str.len() + 2;

                if let Some((first, second)) = inner_str.split_once("=") {
                    iter = first.to_string();
                    start = Box::new(Parser::new(second.tokenise()?).parse()?.0);
                } else {
                    env_info("parser", "Failed extracting iter and start.".into());
                    return Err("Failed extracting iter and start".into());
                }
            } else {
                env_info("parser", "Failed extracting iter and start.".into());
                return Err("Failed extracting iter and start".into());
            }
        } else {
            env_info("parser", "Failed extracting iter and start.".into());
            return Err("Failed extracting iter and start".into());
        }

        if let Some(end_tokens) = token_stream.get(len..) {
            if let Some(ext_end_tokens) = Parser::extract_exponent(end_tokens.to_vec()) {
                len += ext_end_tokens.len() + 3;
                end = Box::new(Parser::new(ext_end_tokens).parse()?.0);
            } else {
                env_info("parser", "Failed extracting end.".into());
                return Err("Failed extracting end".into());
            }
        } else {
            env_info("parser", "Failed extracting end.".into());
            return Err("Failed extracting end".into());
        }

        env_info("parser", "Extracting inner math...".into());
        if let Some(inner_math_stream) = token_stream.get(len..) {
            if let Some(inner_math_inner) = Parser::extract_between(
                &inner_math_stream.to_vec(),
                TokenType::CurlyOpen,
                TokenType::CurlyClose,
            ) {
                len += inner_math_inner.len() + 2;
                inner = Box::new(Parser::new(inner_math_inner.to_vec()).parse()?.0);
            } else {
                env_info("parser", "Failed extracting inner math.".into());
                return Err("Failed extracting inner math".into());
            }
        } else {
            env_info("parser", "Failed extracting inner math.".into());
            return Err("Failed extracting inner math".into());
        }

        return Ok((
            PolynomPart::Math(Math::Sum(Sum {
                inner,
                start,
                end,
                iter,
            })),
            len,
        ));
    }
}
