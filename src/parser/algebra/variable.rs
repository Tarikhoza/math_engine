use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use crate::logging::{env_error, env_info, env_warn};

use crate::castable::Castable;
use crate::lexer::{Token, TokenType, Tokenisable};
use crate::math::algebra::polynom::PolynomPart;
use crate::math::algebra::variable::Variable;
use crate::math::Math;
use crate::parser::{Parsable, ParsablePrimitiveAsVariable, Parser};

impl Parsable for Variable {
    fn to_tex(&self) -> String {
        if self.value.is_zero() {
            return String::from("0");
        }
        let mut val = self.value.normalize().to_string();
        if (val == "1.0" || val == "1") && !self.suffix.is_empty() {
            val = String::new();
        } else if (val == "-1.0" || val == "-1") && !self.suffix.is_empty() {
            val = String::from("-");
        }
        match &self.exponent {
            Some(has_exp) => match &has_exp.to_tex()[..] {
                "1.0" | "1" => format!("{}{}", val, self.suffix),
                exp => format!("{}{}^{{{}}}", val, self.suffix, exp),
            },

            _no_exp => format!("{}{}", val, self.suffix),
        }
    }

    fn from_token_stream(token_stream: Vec<Token>) -> Result<(PolynomPart, usize), String> {
        env_info("parser", "Trying to parse token stream as variable".into());
        let token_types = token_stream
            .iter()
            .map(|token| token.token_type.clone())
            .collect::<Vec<TokenType>>();

        let mut len = 0;

        let value: Decimal;
        let mut suffix: String = "".into();
        let mut exponent: Option<Box<Math>> = None;

        match token_types.as_slice() {
            //Should match for example 22.1a or 3.4\\alpha
            [TokenType::Number, TokenType::DecimalPoint, TokenType::Number, TokenType::LowerLetter | TokenType::GreekLetter, ..] =>
            {
                env_info(
                    "parser",
                    "Variable - matched pattern decimal with suffix".into(),
                );
                let whole_str: String = token_stream[0].literal.clone().unwrap_or_default();
                let decimal_str: String = token_stream[2].literal.clone().unwrap_or_default();
                suffix = token_stream[3].literal.clone().unwrap_or_default();
                if whole_str.is_empty() || decimal_str.is_empty() || suffix.is_empty() {
                    return Err("Whole, decimal or suffix are empty".to_string());
                }
                if let Ok(dec) = Decimal::from_str_exact(&format!("{}.{}", whole_str, decimal_str))
                {
                    value = dec;
                } else {
                    return Err("Failed converting token stream to variable".to_string());
                }
                len += 4;
            }
            // Should match for example 2.1 or 3.4
            [TokenType::Number, TokenType::DecimalPoint, TokenType::Number, ..] => {
                env_info("parser", "Variable - matched pattern decimal".into());
                let whole_str: String = token_stream[0].literal.clone().unwrap_or_default();
                let decimal_str: String = token_stream[2].literal.clone().unwrap_or_default();

                if whole_str.is_empty() || decimal_str.is_empty() {
                    return Err("Whole or decimal are empty".to_string());
                }
                if let Ok(dec) = Decimal::from_str_exact(&format!("{}.{}", whole_str, decimal_str))
                {
                    value = dec;
                } else {
                    return Err("Failed converting token stream to variable".to_string());
                }
                len += 3;
            }
            // Should match for example 3a or 5\\alpha
            [TokenType::Number, TokenType::LowerLetter | TokenType::GreekLetter, ..] => {
                env_info(
                    "parser",
                    "Variable - matched pattern number with suffix".into(),
                );
                let whole_str: String = token_stream[0].literal.clone().unwrap_or_default();
                suffix = token_stream[1].literal.clone().unwrap_or_default();

                if whole_str.is_empty() || suffix.is_empty() {
                    return Err("Whole or suffix are empty".to_string());
                }
                if let Ok(dec) = Decimal::from_str_exact(&whole_str) {
                    value = dec;
                } else {
                    return Err("Failed converting token stream to variable".to_string());
                }
                len += 2;
            }

            // Should match for example a,b,c, \\alpha...
            [TokenType::LowerLetter | TokenType::GreekLetter, ..] => {
                env_info(
                    "parser",
                    "Variable - matched pattern just suffix suffix".into(),
                );
                suffix = token_stream[0].literal.clone().unwrap_or_default();
                if suffix.is_empty() {
                    return Err("Suffix is empty".to_string());
                }

                value = dec!(1);
                len += 1;
            }
            // Should match for example 1, 2, 3, 4, 6, 7, 8, 9
            [TokenType::Number, ..] => {
                env_info("parser", "Variable - matched pattern number".into());
                let whole_str: String = token_stream[0].literal.clone().unwrap_or_default();
                if whole_str.is_empty() {
                    return Err("Whole is empty".to_string());
                }
                if let Ok(dec) = Decimal::from_str_exact(&whole_str) {
                    value = dec;
                } else {
                    return Err("Failed converting token stream to variable".to_string());
                }
                len += 1;
            }

            _ => return Err("Does not match any variable pattern".to_string()),
        }

        if let Some(exponent_tokens) = token_stream.get(len..) {
            if let Some(ext_exp_tokens) = Parser::extract_exponent(exponent_tokens.to_vec()) {
                len += ext_exp_tokens.len() + 3;
                exponent = Some(Box::new(Parser::new(ext_exp_tokens).parse()?.0));
            }
        }

        env_info(
            "parser",
            format!(
                "Variable - parsed variable as {} {} {:#?}",
                value.clone(),
                suffix.clone(),
                exponent.clone(),
            ),
        );

        Ok((
            Math::as_polynom_part(
                &Variable {
                    value,
                    suffix,
                    exponent,
                }
                .as_math(),
            ),
            len,
        ))
    }
}
