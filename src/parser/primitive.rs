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

impl ParsablePrimitiveAsVariable for i64 {
    fn as_variable(&self) -> Variable {
        Variable {
            value: Decimal::new(*self, 0),
            suffix: String::new(),
            exponent: None,
        }
    }
}

impl ParsablePrimitiveAsVariable for Decimal {
    fn as_variable(&self) -> Variable {
        Variable {
            value: *self,
            suffix: String::new(),
            exponent: None,
        }
    }
}
