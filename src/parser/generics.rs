use crate::math::Math;
use crate::parser::{ParsableGenerics, Parser};

impl ParsableGenerics for String {
    fn parse_math(&self) -> Result<Math, &'static str> {
        Parser::new(self).parse()
    }
}
impl ParsableGenerics for str{
    fn parse_math(&self) -> Result<Math, &'static str> {
        Parser::new(self).parse()
    }
}
impl ParsableGenerics for usize {
    fn parse_math(&self) -> Result<Math, &'static str> {
        self.to_string().parse_math()
    }
}
impl ParsableGenerics for f32 {
    fn parse_math(&self) -> Result<Math, &'static str> {
        self.to_string().parse_math()
    }
}

impl ParsableGenerics for f64 {
    fn parse_math(&self) -> Result<Math, &'static str> {
        self.to_string().parse_math()
    }
}
