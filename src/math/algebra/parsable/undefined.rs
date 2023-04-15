use crate::math::algebra::undefined::Undefined;
use crate::parser::Parsable;

impl Parsable for Undefined {
    fn to_tex(&self) -> String {
        String::from("undefined")
    }
}
