use crate::math::algebra::fraction::Fraction;
use crate::math::Math;
use crate::parser::{Parsable, ParsableGenerics, Parser};
use fancy_regex::Regex;

impl Parsable for Fraction {
    fn to_tex(&self) -> String {
        format!(
            "\\frac{{{}}}{{{}}}",
            self.numerator.to_tex(),
            self.denominator.to_tex()
        )
    }

    fn from_tex(tex: &str) -> Result<Math, &'static str> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\\frac)(.*)").unwrap_or_else(|e| {
                panic!("Failed to compile regex for braces: {e}");
            });
        }
        let result = RE.captures(tex);
        let captures = result
            .expect("Error running regex")
            .expect("No match found");

        let numerator =
            Parser::extract_brace(captures.get(2).map_or("", |m| m.as_str()), '{', '}')?;
        let denominator = Parser::extract_brace(
            captures
                .get(2)
                .map_or("", |m| m.as_str())
                .get(numerator.len() + 2..)
                .unwrap(),
            '{',
            '}',
        )?;

        dbg!(numerator.clone(), denominator.clone());
        if numerator.is_empty() {
            return Err("While parsing numerator was empty");
        }
        if denominator.is_empty() {
            return Err("While parsing denominator was empty");
        }

        return Ok(Math::Fraction(Fraction {
            numerator: Box::new(numerator.parse_math()?),
            denominator: Box::new(denominator.parse_math()?),
        }));
    }

    fn on_begining(tex: String) -> Option<String> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(\\frac)(.*)").unwrap_or_else(|e| {
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
