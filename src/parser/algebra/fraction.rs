use crate::logging::{env_error, env_info, env_warn};

use crate::castable::Castable;
use crate::lexer::{Token, TokenType, Tokenisable};
use crate::math::algebra::fraction::Fraction;
use crate::math::algebra::polynom::PolynomPart;
use crate::math::Math;
use crate::parser::{Parsable, ParsablePrimitiveAsVariable, ParsableStream, Parser};

impl Parsable for Fraction {
    fn to_tex(&self) -> String {
        if let Some(whole) = self.whole {
            return format!(
                "{}\\frac{{{}}}{{{}}}",
                whole,
                self.numerator.to_tex(),
                self.denominator.to_tex()
            );
        }
        format!(
            "\\frac{{{}}}{{{}}}",
            self.numerator.to_tex(),
            self.denominator.to_tex()
        )
    }

    fn from_token_stream(token_stream: Vec<Token>) -> Result<(PolynomPart, usize), String> {
        env_info("parser", "Trying to parse token stream as fraction".into());
        let mut len = 1;
        let denominator: Vec<Token>;
        let numerator: Vec<Token>;

        env_info("parser", "Looking for FracStart token".into());
        if let Some(token) = token_stream.first() {
            if token.token_type != TokenType::FracStart {
                return Err("No fraction token on begining".into());
            }
        } else {
            return Err("No fraction token on begining".into());
        }

        env_info("parser", "Extracting numerator...".into());
        if let Some(num_stream) = token_stream.get(len..) {
            if let Some(num_inner) = Parser::extract_between(
                &num_stream.to_vec(),
                TokenType::CurlyOpen,
                TokenType::CurlyClose,
            ) {
                numerator = num_inner;
                len += numerator.len() + 2;
            } else {
                return Err("Failed extracting numerator".into());
            }
        } else {
            return Err("Failed extracting numerator".into());
        }

        env_info("parser", "Extracting denominator...".into());
        if let Some(denom_stream) = token_stream.get(len..) {
            if let Some(denom_inner) = Parser::extract_between(
                &denom_stream.to_vec(),
                TokenType::CurlyOpen,
                TokenType::CurlyClose,
            ) {
                denominator = denom_inner;
                len += denominator.len() + 2;
            } else {
                return Err("Failed extracting denominator".into());
            }
        } else {
            return Err("Failed extracting denominator".into());
        }

        return Ok((
            PolynomPart::Math(
                Fraction {
                    whole: None,
                    numerator: Box::new(Parser::new(numerator).parse()?),
                    denominator: Box::new(Parser::new(denominator).parse()?),
                }
                .as_math(),
            ),
            len,
        ));
    }
}
