use crate::math::algebra::braces::Braces;
use crate::math::algebra::variable::Variable;
use crate::math::Math;
use crate::parser::{Parsable, Parser};
use fancy_regex::Regex;
use rust_decimal_macros::dec;

impl Parsable for Braces {
    #[must_use]
    fn to_tex(&self) -> String {
        match &self.exponent {
            Some(has_exp) => match &has_exp.to_tex()[..] {
                "1.0" | "1" => format!("({})", &self.math.to_tex()),
                exp => format!("({})^{{{}}}", &self.math.to_tex(), exp),
            },
            _no_exp => format!("({})", &self.math.to_tex()),
        }
    }

    fn from_tex(tex: &str) -> Result<Math, &'static str> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?<!_)\((.+)").unwrap_or_else(|e| {
                panic!("Failed to compile regex for braces: {e}");
            });
        }
        let result = RE.captures(tex);
        let captures = result
            .expect("Error running regex")
            .expect("No match found");
        let math = Parser::extract_brace(&captures.get(0).map_or("", |m| m.as_str()), '(', ')');
        let exponent_str = tex.split_at(math.len() + 2).1;

        let exponent: Option<Box<Math>> = if !exponent_str.is_empty()
            && exponent_str.starts_with('^')
            && exponent_str.chars().nth(1) == Some('{')
        {
            Some(Box::new(
                Parser::new(&(Parser::extract_brace(&exponent_str[1..], '{', '}'))).parse()?,
            ))
        } else {
            None
        };

        Ok(Math::Braces(Braces {
            math: Box::new(Parser::new(&math).parse()?),
            exponent,
        }))
    }

    fn on_begining(tex: String) -> Option<String> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(?<!_)\((.+)").unwrap_or_else(|e| {
                panic!("Failed to compile regex for braces: {e}");
            });
        }

        if let Ok(Some(f)) = RE.find(&tex) {
            let f_str = f.as_str().to_string();
            if !f_str.is_empty() {
                return Some(f_str);
            }
        }
        None
    }
}
