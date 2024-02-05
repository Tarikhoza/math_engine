use itertools::Itertools;

use crate::definitions::math_functions::find_function_definition;
use crate::logging::{env_error, env_info, env_warn};

use crate::castable::Castable;
use crate::lexer::{Token, TokenType, Tokenisable};
use crate::math::algebra::absolute::Absolute;
use crate::math::algebra::exponentable::Exponentable;
use crate::math::algebra::function::{Function, FunctionDefinition, FunctionInstance};
use crate::math::algebra::polynom::PolynomPart;
use crate::math::Math;
use crate::parser::{
    Parsable, ParsablePrimitive, ParsablePrimitiveAsVariable, ParsableStream, Parser,
};

impl Parsable for Function {
    fn to_tex(&self) -> String {
        match self {
            Function::FunctionInstance(f) => {
                let exponent = f.get_exponent();
                if exponent.to_tex() != *"1" && exponent.to_tex() != *"" {
                    return format!(
                        "\\mathrm{{{}}}({})^{{{}}}",
                        f.label,
                        f.args.iter().map(|a| a.to_tex()).join(","),
                        exponent.to_tex()
                    );
                }
                format!(
                    "\\mathrm{{{}}}({})",
                    f.label,
                    f.args.iter().map(|a| a.to_tex()).join(",")
                )
            }
            Function::FunctionDefinition(FunctionDefinition::FunctionalDefinition(f)) => {
                return format!("\\mathrm{{{}}}({})", f.label, f.args.join(","),);
            }
            Function::FunctionDefinition(FunctionDefinition::MappingDefinition(f)) => {
                return format!("\\mathrm{{{}}}({})", f.label, f.args.join(","),);
            }
        }
    }

    fn from_token_stream(token_stream: Vec<Token>) -> Result<(PolynomPart, usize), String> {
        env_info("parser", "Trying to parse token stream as function".into());
        let mut len = 1;
        let label: String;
        let args: Vec<Token>;

        env_info("parser", "Looking for FuncStart token".into());
        if let Some(token) = token_stream.first() {
            if token.token_type != TokenType::FuncStart {
                return Err("No function token on begining".into());
            }
        } else {
            return Err("No function token on begining".into());
        }

        env_info("parser", "Extracting label...".into());
        if let Some(label_stream) = token_stream.get(len..) {
            if let Some(label_inner) = Parser::extract_between(
                &label_stream.to_vec(),
                TokenType::CurlyOpen,
                TokenType::CurlyClose,
            ) {
                label = label_inner
                    .iter()
                    .map(|token| token.literal.clone().unwrap_or_default())
                    .collect::<Vec<String>>()
                    .concat();

                len += label.len() + 2;
            } else {
                return Err("Failed extracting label".into());
            }
        } else {
            return Err("Failed extracting label".into());
        }

        env_info("parser", "Extracting arguments...".into());
        if let Some(args_stream) = token_stream.get(len..) {
            if let Some(args_inner) = Parser::extract_between(
                &args_stream.to_vec(),
                TokenType::CurlyOpen,
                TokenType::CurlyClose,
            ) {
                args = args_inner;
                len += args.len() + 2;
            } else {
                return Err("Failed extracting function arguments".into());
            }
        } else {
            return Err("Failed extracting function arguments".into());
        }

        let mut exponent: Option<Box<Math>> = None;
        if let Some(exponent_tokens) = token_stream.get(len..) {
            if let Some(ext_exp_tokens) = Parser::extract_exponent(exponent_tokens.to_vec()) {
                len += ext_exp_tokens.len() + 3;
                exponent = Some(Box::new(Parser::new(ext_exp_tokens).parse()?));
            }
        }

        return Ok((
            PolynomPart::Math(Math::Function(Function::FunctionInstance(
                FunctionInstance {
                    label: label.clone(),
                    args: args
                        .iter()
                        .map(|token| token.literal.clone().unwrap_or_default())
                        .collect::<Vec<String>>()
                        .concat()
                        .split(",")
                        .map(|str| str.parse_math().unwrap())
                        .collect::<Vec<Math>>(),
                    //TODO remove the unwrap above
                    exponent,
                    definition: find_function_definition(&label.clone()),
                },
            ))),
            len,
        ));
    }
}
