use crate::math::Math;
use crate::parser::{ParsableGenerics, Parser};

impl ParsableGenerics for String {
    fn parse_math(&self) -> Result<Math, &'static str> {
        Parser::new(self).parse()
    }
}
