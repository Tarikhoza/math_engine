use crate::math::algebra::variable::Variable;
use crate::math::Math;
use crate::parser::{ParsableGenerics, ParsableGenericsAsVariable, Parser};

impl ParsableGenerics for String {
    fn parse_math(&self) -> Result<Math, &'static str> {
        Parser::new(self).parse()
    }
}

impl ParsableGenerics for str {
    fn parse_math(&self) -> Result<Math, &'static str> {
        Parser::new(self).parse()
    }
}

impl ParsableGenericsAsVariable for usize {
    fn parse_math(&self) -> Result<Math, &'static str> {
        self.to_string().parse_math()
    }

    fn as_variable(&self) -> Variable {
        match self.parse_math().expect("error parsing usize as variable") {
            Math::Variable(v) => return v,
            _ => panic!("error parsing usize as variable"),
        }
    }
}

impl ParsableGenericsAsVariable for i64 {
    fn parse_math(&self) -> Result<Math, &'static str> {
        self.to_string().parse_math()
    }

    fn as_variable(&self) -> Variable {
        match self.parse_math().expect("error parsing usize as variable") {
            Math::Variable(v) => return v,
            _ => panic!("error parsing usize as variable"),
        }
    }
}

impl ParsableGenericsAsVariable for f32 {
    fn parse_math(&self) -> Result<Math, &'static str> {
        self.to_string().parse_math()
    }

    fn as_variable(&self) -> Variable {
        match self.parse_math().expect("error parsing usize as variable") {
            Math::Variable(v) => return v,
            _ => panic!("error parsing usize as variable"),
        }
    }
}

impl ParsableGenericsAsVariable for i32 {
    fn parse_math(&self) -> Result<Math, &'static str> {
        self.to_string().parse_math()
    }

    fn as_variable(&self) -> Variable {
        match self.parse_math().expect("error parsing usize as variable") {
            Math::Variable(v) => return v,
            _ => panic!("error parsing usize as variable"),
        }
    }
}

impl ParsableGenericsAsVariable for f64 {
    fn parse_math(&self) -> Result<Math, &'static str> {
        self.to_string().parse_math()
    }

    fn as_variable(&self) -> Variable {
        match self.parse_math().expect("error parsing usize as variable") {
            Math::Variable(v) => return v,
            _ => panic!("error parsing usize as variable"),
        }
    }
}
