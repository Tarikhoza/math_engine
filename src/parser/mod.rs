pub mod algebra;
pub mod calculus;
pub mod math;
pub mod primitive;

use crate::{
    castable::Castable,
    lexer::{Token, TokenType, Tokenisable},
    math::{
        algebra::{
            absolute::Absolute,
            braces::Braces,
            fraction::Fraction,
            function::Function,
            operations::Operator as AlgebraOperator,
            polynom::{Polynom, PolynomPart},
            root::Root,
            variable::Variable,
        },
        calculus::sum::Sum,
        calculus::{factorial::Factorial, product::Product},
        Math,
    },
};

use crate::logging::{env_error, env_info, env_warn};
use colored::Colorize;

pub struct Parser {
    token_stream: Vec<Token>,
    pos: usize,
}

pub trait Parsable {
    fn to_tex(&self) -> String;
    fn from_tex(tex: &str) -> Result<PolynomPart, String> {
        let token_stream = tex.tokenise();
        if let Ok(tokens) = token_stream {
            let ret = Self::from_token_stream(tokens);
            if let Ok((math, len)) = ret {
                return Ok(math);
            } else if let Err(err) = ret {
                return Err(format!("from_tex(\"{}\") failed: {}", tex, err));
            } else {
                return Err(format!("from_tex(\"{}\") failed", tex));
            }
        } else if let Err(err) = token_stream {
            return Err(format!("from_tex(\"{}\") failed: {}", tex, err));
        } else {
            return Err(format!("from_tex(\"{}\") failed", tex));
        }
    }
    fn from_token_stream(token_stream: Vec<Token>) -> Result<(PolynomPart, usize), String>;
}

pub trait ParsablePrimitive {
    fn parse_math(&self) -> Result<Math, String>;
}

pub trait ParsablePrimitiveAsVariable {
    fn as_variable(&self) -> Variable;
}

pub trait ParsableStream {
    fn parse(&self) -> Result<(PolynomPart, usize), String>;
}

impl ParsableStream for Vec<Token> {
    fn parse(&self) -> Result<(PolynomPart, usize), String> {
        Math::from_token_stream(self.to_vec())
    }
}

#[derive(PartialEq)]
enum ParsableType {
    Math,
    Suffix,
    Operator,
    None,
}

impl Parser {
    pub fn new(token_stream: Vec<Token>) -> Parser {
        Parser {
            token_stream,
            pos: 0,
        }
    }

    pub fn extract_between(
        token_stream: &Vec<Token>,
        t_open: TokenType,
        t_close: TokenType,
    ) -> Option<Vec<Token>> {
        env_info(
            "parser",
            format!(
                "Trying to extract tokens between {:#?} and {:#?}",
                t_open, t_close
            ),
        );
        let mut pos = 0;
        let mut close = 0;
        let mut open = 0;
        if let Some(token) = token_stream.first() {
            if token.token_type != t_open {
                env_info(
                    "parser",
                    format!("First token is not open token {:#?}", token),
                );
                return None;
            }
        }

        while pos < token_stream.len() {
            if let Some(current_token) = token_stream.get(pos) {
                match &current_token.token_type {
                    t if &t_open == t => {
                        env_info("parser", "Found open token".into());
                        open += 1;
                    }
                    t if &t_close == t => {
                        env_info("parser", "Found close token".into());
                        close += 1;
                    }
                    _ => env_info("parser", "Some".into()),
                }
            }
            if open == close {
                env_info("parser", "Open same as close".into());
                return if let Some(inner) = token_stream.get(1..pos) {
                    Some(inner.to_vec())
                } else {
                    None
                };
            }
            pos += 1;
        }
        None
    }

    pub fn extract_exponent(token_stream: Vec<Token>) -> Option<Vec<Token>> {
        if token_stream
            .first()
            .is_some_and(|x| x.token_type == TokenType::Caret)
        {
            env_info("parser", "Trying to extract exponent".into());
            if let Some(exponent_tokens) = Parser::extract_between(
                &token_stream[1..].to_vec(),
                TokenType::CurlyOpen,
                TokenType::CurlyClose,
            ) {
                return Some(exponent_tokens);
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn parse(&mut self) -> Result<Math, String> {
        let math_prasing_functions = vec![
            Variable::from_token_stream,
            Braces::from_token_stream,
            Absolute::from_token_stream,
            Fraction::from_token_stream,
            Function::from_token_stream,
            Root::from_token_stream,
            Product::from_token_stream,
            Sum::from_token_stream,
        ];
        let operator_parsing_functions = vec![AlgebraOperator::from_token_stream];

        let mut last_type = ParsableType::None;
        let mut parts: Vec<PolynomPart> = Vec::new();

        env_info("parser", "Starting parser\n".into());
        'outer: while (self.pos < self.token_stream.len()) {
            if let Some(current_stream) = self.token_stream.get(self.pos..) {
                env_info(
                    "parser",
                    format!(
                        "Parsing token stream at position {},\n{:#?}",
                        self.pos, current_stream
                    ),
                );
                if let Some(first) = current_stream.first() {
                    if first.token_type == TokenType::EOF {
                        env_info("parser", "EOF token is remaining, stopping parser".into());
                        break;
                    }
                    //if suffix then inject pevious into suffix
                    if first.token_type == TokenType::Bang {
                        if let Some(PolynomPart::Math(prev)) = parts.pop() {
                            parts.push(
                                Factorial {
                                    inner: Box::new(prev),
                                }
                                .as_math()
                                .as_polynom_part(),
                            );
                            self.pos += 1;
                            last_type = ParsableType::Suffix;
                            continue 'outer;
                        } else {
                            env_info("parser", "Injecting into suffix failed".into());
                        }
                    }
                }

                for try_parse in operator_parsing_functions.as_slice() {
                    if let Ok((op, len)) = try_parse(current_stream.to_vec()) {
                        env_info("parser", "Parsed as operator".into());
                        parts.push(op);
                        self.pos += len;
                        last_type = ParsableType::Operator;
                        continue 'outer;
                    }
                }
                for try_parse in math_prasing_functions.as_slice() {
                    if let Ok((math, len)) = try_parse(current_stream.to_vec()) {
                        env_info("parser", "Parsed as math".into());
                        if last_type != ParsableType::Operator && last_type != ParsableType::None {
                            parts.push(PolynomPart::Operator(AlgebraOperator::InvMulti));
                        }
                        parts.push(math);
                        self.pos += len;
                        last_type = ParsableType::Math;
                        continue 'outer;
                    }
                }

                env_error("parser", "Could not match anything. Parsing failed.".into());
            }

            let mut error_stream = self
                .token_stream
                .iter()
                .map(|token| token.literal.clone().unwrap_or_default())
                .collect::<Vec<String>>();

            let errored = error_stream[self.pos].clone();

            error_stream.remove(self.pos);
            error_stream.insert(self.pos, errored.red().to_string());

            let input = error_stream.clone().concat();

            return Err(format!(
                "Error parsing token_stream.\n Token stream: {:#?},\nAlready parsed: {:#?},\nRemaining to parse: {:#?},\nCould not parse token at position {},\nTokenType that caused error:\n{:#?},\nInput:'{}',\nCaused parsing error:{}\n",
                self.token_stream,
                parts,
                self.token_stream.get(self.pos..),
                self.pos,
                self.token_stream[self.pos],
                input,
                error_stream.concat()
            ));
        }
        env_info("parser", format!("Succesfuly parsed {:#?}", parts));

        Ok(Polynom { parts }.as_math())
    }
}
