use crate::math::Math;
use crate::parser::{Parsable, Parser};

impl Parsable for Math {
    #[must_use]
    fn to_tex(&self) -> String {
        match self {
            Math::Variable(s) => s.to_tex(),
            Math::Braces(s) => s.to_tex(),
            Math::Polynom(s) => s.to_tex(),
            Math::Undefined(s) => s.to_tex(),
            Math::Operator(s) => s.to_tex(),
            _ => todo!(),
        }
    }

    fn from_tex(tex: &str) -> Result<Math, &'static str> {
        Parser::new(tex).parse()
    }

    #[must_use]
    fn on_begining(_tex: String) -> Option<String> {
        None
    }
}
