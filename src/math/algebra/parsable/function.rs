use crate::math::algebra::exponentable::Exponentable;
use crate::math::algebra::function::Function;
use crate::math::Math;
use crate::parser::{Parsable, ParsablePrimitive, ParsablePrimitiveAsVariable, Parser};

use fancy_regex::Regex;

impl Parsable for Function {
    fn to_tex(&self) -> String {
        let mut ret = "";
        let exponent = self.get_exponent();

        if exponent.to_tex() != *"1" && exponent.to_tex() != *"" {
            return format!(
                "\\mathrm{{{}}}({})^{{{}}}",
                self.label,
                self.args.join(","),
                exponent.to_tex()
            );
        }
        format!("\\mathrm{{{}}}({})", self.label, self.args.join(","))
    }

    fn from_tex(tex: &str) -> Result<Math, &'static str> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\\mathrm)(.*)").unwrap_or_else(|e| {
                panic!("Failed to compile regex for braces: {e}");
            });
        }

        let result = RE.captures(tex);
        let captures = result
            .expect("Error running regex")
            .expect("No match found");

        let label = Parser::extract_brace(captures.get(2).map_or("", |m| m.as_str()), '{', '}')?;

        let arguments = Parser::extract_brace(
            captures
                .get(2)
                .map_or("", |m| m.as_str())
                .get(label.len() + 2..)
                .ok_or("Error parsing argument of function")?,
            '(',
            ')',
        )?;

        let mut exponent_braces = captures
            .get(2)
            .map_or("", |m| m.as_str())
            .get(label.len() + 2 + arguments.len() + 3..);

        let mut exponent = String::from("");
        if let Some(brac) = exponent_braces {
            exponent = Parser::extract_brace(brac, '{', '}')?;
        }

        //TODO check in function database if there is a function with the same label
        //If yes overwrite definition with database definition
        //else await an definition after exponent

        if !exponent.is_empty() {
            Ok(Math::Function(Function {
                label,
                args: arguments
                    .replace(' ', "")
                    .split(',')
                    .map(|s| s.to_string())
                    .collect(),
                definition: None,
                exponent: Some(Box::new(
                    exponent
                        .parse_math()
                        .expect("failed parsing exponent as math"),
                )),
            }))
        } else {
            Ok(Math::Function(Function {
                label,
                args: arguments
                    .replace(' ', "")
                    .split(',')
                    .map(|s| s.to_string())
                    .collect(),
                definition: None,
                exponent: None,
            }))
        }
    }

    fn from_tex_len(tex: &str) -> Result<(usize, Math), &'static str> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\\mathrm)(.*)").unwrap_or_else(|e| {
                panic!("Failed to compile regex for braces: {e}");
            });
        }

        let mut len = 7;

        let result = RE.captures(tex);
        let captures = result
            .expect("Error running regex")
            .expect("No match found");

        let label = Parser::extract_brace(captures.get(2).map_or("", |m| m.as_str()), '{', '}')?;

        len += label.len() + 2;

        let arguments = Parser::extract_brace(
            captures
                .get(2)
                .map_or("", |m| m.as_str())
                .get(label.len() + 2..)
                .ok_or("Error parsing argument of function")?,
            '(',
            ')',
        )?;

        len += arguments.len() + 2;

        let mut exponent_braces = captures
            .get(2)
            .map_or("", |m| m.as_str())
            .get(label.len() + 2 + arguments.len() + 3..);

        let mut exponent = String::from("");
        if let Some(brac) = exponent_braces {
            len += brac.len();
            exponent = Parser::extract_brace(brac, '{', '}')?;
        }

        //TODO check in function database if there is a function with the same label
        //If yes overwrite definition with database definition
        //else await an definition after exponent

        if !exponent.is_empty() {
            Ok((
                len,
                Math::Function(Function {
                    label,
                    args: arguments
                        .replace(' ', "")
                        .split(',')
                        .map(|s| s.to_string())
                        .collect(),
                    definition: None,
                    exponent: Some(Box::new(
                        exponent
                            .parse_math()
                            .expect("failed parsing exponent as math"),
                    )),
                }),
            ))
        } else {
            Ok((
                len,
                Math::Function(Function {
                    label,
                    args: arguments
                        .replace(' ', "")
                        .split(',')
                        .map(|s| s.to_string())
                        .collect(),
                    definition: None,
                    exponent: None,
                }),
            ))
        }
    }

    fn on_begining(tex: String) -> Option<String> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(\\mathrm)(.*)").unwrap_or_else(|e| {
                panic!("Failed to compile regex for braces: {e}");
            });
        }

        if let Ok(Some(f)) = RE.find(&tex) {
            if !f.as_str().to_string().is_empty() {
                return Some(f.as_str().to_string());
            }
        }
        None
    }
}
