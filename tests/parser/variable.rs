use math_engine::parser::{Parsable, ParsablePrimitive, ParsablePrimitiveAsVariable, Parser};

#[test]
fn parse() {
    Variable::from_tex("1.0");
    Variable::from_tex("a");
    Variable::from_tex("3x");
    Variable::from_tex("3.14x");
    Variable::from_tex("10^{10}");
}
