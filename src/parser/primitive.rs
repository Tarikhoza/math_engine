use crate::math::algebra::variable::Variable;
use crate::math::Math;
use crate::parser::{ParsablePrimitive, ParsablePrimitiveAsVariable, Parser};

use rust_decimal::Decimal;

impl ParsablePrimitive for String {
    fn parse_math(&self) -> Result<Math, &'static str> {
        Parser::new(self).parse()
    }
}

impl ParsablePrimitive for str {
    fn parse_math(&self) -> Result<Math, &'static str> {
        Parser::new(self).parse()
    }
}

impl ParsablePrimitiveAsVariable for usize {
    fn parse_math(&self) -> Result<Math, &'static str> {
        self.to_string().parse_math()
    }

    fn as_variable(&self) -> Variable {
        match self.parse_math().expect("error parsing usize as variable") {
            Math::Variable(v) => v,
            _ => panic!("error parsing usize as variable"),
        }
    }
}

impl ParsablePrimitiveAsVariable for i64 {
    fn parse_math(&self) -> Result<Math, &'static str> {
        self.to_string().parse_math()
    }

    fn as_variable(&self) -> Variable {
        match self.parse_math().expect("error parsing usize as variable") {
            Math::Variable(v) => v,
            _ => panic!("error parsing usize as variable"),
        }
    }
}

impl ParsablePrimitiveAsVariable for f32 {
    fn parse_math(&self) -> Result<Math, &'static str> {
        self.to_string().parse_math()
    }

    fn as_variable(&self) -> Variable {
        match self.parse_math().expect("error parsing usize as variable") {
            Math::Variable(v) => v,
            _ => panic!("error parsing usize as variable"),
        }
    }
}

impl ParsablePrimitiveAsVariable for i32 {
    fn parse_math(&self) -> Result<Math, &'static str> {
        self.to_string().parse_math()
    }

    fn as_variable(&self) -> Variable {
        match self.parse_math().expect("error parsing usize as variable") {
            Math::Variable(v) => v,
            _ => panic!("error parsing usize as variable"),
        }
    }
}

impl ParsablePrimitiveAsVariable for f64 {
    fn parse_math(&self) -> Result<Math, &'static str> {
        self.to_string().parse_math()
    }

    fn as_variable(&self) -> Variable {
        match self.parse_math().expect("error parsing usize as variable") {
            Math::Variable(v) => v,
            _ => panic!("error parsing usize as variable"),
        }
    }
}

impl ParsablePrimitiveAsVariable for Decimal {
    fn parse_math(&self) -> Result<Math, &'static str> {
        self.to_string().parse_math()
    }

    fn as_variable(&self) -> Variable {
        match self.parse_math().expect("error parsing usize as variable") {
            Math::Variable(v) => v,
            _ => panic!("error parsing usize as variable"),
        }
    }
}
