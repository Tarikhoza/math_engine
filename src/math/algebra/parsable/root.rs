use crate::math::algebra::root::Root;
use crate::math::Math;
use crate::parser::{Parsable, ParsableGenerics, Parser};
use fancy_regex::Regex;

impl Parsable for Root {
    fn to_tex(&self) -> String {
        if self.base.is_none() || self.base.clone().unwrap().to_tex() == "2" {
            format!("\\sqrt[]{{{}}}", self.math.to_tex())
        } else {
            format!(
                "\\sqrt[{}]{{{}}}",
                self.base.clone().unwrap().to_tex(),
                self.math.to_tex()
            )
        }
    }

    fn from_tex(tex: &str) -> Result<Math, &'static str> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\\sqrt)(.*)").unwrap_or_else(|e| {
                panic!("Failed to compile regex for braces: {e}");
            });
        }
        let result = RE.captures(tex);
        let captures = result
            .expect("Error running regex")
            .expect("No match found");

        let base = Parser::extract_brace(captures.get(2).map_or("", |m| m.as_str()), '[', ']')?;
        let math = Parser::extract_brace(
            captures
                .get(2)
                .map_or("", |m| m.as_str())
                .get(base.len() + 2..)
                .unwrap(),
            '{',
            '}',
        )?;
        if base.is_empty() {
            return Ok(Math::Root(Root {
                math: Box::new(math.parse_math()?),
                base: None,
            }));
        }
        Ok(Math::Root(Root {
            math: Box::new(math.parse_math()?),
            base: Some(Box::new(base.parse_math()?)),
        }))
    }

    fn on_begining(tex: String) -> Option<String> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(\\sqrt)(.*)").unwrap_or_else(|e| {
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
