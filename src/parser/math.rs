use crate::math::Math;
use crate::parser::{Parsable, Parser};

impl Parsable for Math {
    fn to_tex(&self) -> String {
        match self {
            Math::Variable(s) => s.to_tex(),
            Math::Fraction(s) => s.to_tex(),
            Math::Braces(s) => s.to_tex(),
            Math::Polynom(s) => s.to_tex(),
            Math::Undefined(s) => s.to_tex(),
            Math::Infinity(s) => s.to_tex(),
            Math::Root(s) => s.to_tex(),
            Math::Operator(s) => s.to_tex(),
            _ => todo!(),
        }
    }

    fn from_tex(tex: &str) -> Result<Math, &'static str> {
        Parser::new(tex).parse()
    }

    fn on_begining(_tex: String) -> Option<String> {
        None
    }
}
