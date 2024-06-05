use crate::math::algebra::function::{FunctionDefinition, FunctionalDefinition, MappingDefinition};
use crate::math::algebra::variable::Variable;
use crate::math::simplifiable::Simplifiable;
use crate::math::Math;
use crate::parser::ParsablePrimitive;
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

pub fn find_function_definition(label: &str) -> Option<FunctionDefinition> {
    //TODO make this constant
    let func_defs: Vec<FunctionDefinition> = vec![];

    for func in func_defs {
        if func.label() == label {
            return Some(func);
        }
    }
    None
}
