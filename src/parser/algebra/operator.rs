use crate::logging::env_info;
use crate::math::algebra::operations::Operator as AlgebraOperator;
use crate::math::algebra::polynom::PolynomPart;

use crate::lexer::{Token, TokenType};
use crate::parser::Parsable;

impl Parsable for AlgebraOperator {
    fn to_tex(&self) -> String {
        match self {
            AlgebraOperator::Addition => "+".into(),
            AlgebraOperator::Subtraction => "-".into(),
            AlgebraOperator::Multiplication => "*".into(),
            AlgebraOperator::Division => "/".into(),
            AlgebraOperator::InvMulti => "".into(),
            AlgebraOperator::AddSub => "\\pm".into(),
        }
    }

    fn from_token_stream(token_stream: Vec<Token>) -> Result<(PolynomPart, usize), String> {
        env_info(
            "parser",
            format!(
                "Trying to parse token stream {:#?} as AlgebraOperator",
                token_stream
            ),
        );
        let token_types = token_stream
            .iter()
            .map(|token| token.token_type.clone())
            .collect::<Vec<TokenType>>();

        match token_types.as_slice() {
            //Should match for example 22.1a or 3.4\\alpha
            [TokenType::Add, ..] => Ok((PolynomPart::Operator(AlgebraOperator::Addition), 1)),
            [TokenType::Sub, ..] => Ok((PolynomPart::Operator(AlgebraOperator::Subtraction), 1)),
            [TokenType::Mul, ..] => Ok((PolynomPart::Operator(AlgebraOperator::Multiplication), 1)),
            [TokenType::Div, ..] => Ok((PolynomPart::Operator(AlgebraOperator::Division), 1)),
            _ => Err("Could not parse as AlgebraOperator".into()),
        }
    }
}
