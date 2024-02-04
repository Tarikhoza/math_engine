use crate::math::algebra::function::FunctionDefinition;
use crate::math::algebra::variable::Variable;
use crate::math::simplifiable::Simplifiable;
use crate::math::Math;
use once_cell::sync::Lazy;
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

pub fn find_function_definition(label: &str) -> Option<FunctionDefinition> {
    //TODO make this constant
    let func_defs: Vec<FunctionDefinition> = vec![
        //sin
        FunctionDefinition {
            label: "sin".to_string(),
            args: vec!["x".to_string()],
            definition: |args| {
                if args.len() != 1 {
                    panic!("sin takes exactly 1 argument");
                }
                match &args[0].simplify() {
                    // TODO This is just for testing purposes, rewrite this when possible
                    Math::Variable(v) => {
                        if v.is_integer() {
                            return Some(Math::Variable(Variable {
                                value: Decimal::from_f64(v.value.to_f64().unwrap().sin()).unwrap(),
                                suffix: String::from(""),
                                exponent: None,
                            }));
                        }
                        return None;
                    }

                    _ => {}
                }
                None
            },
        },
        FunctionDefinition {
            label: "cos".to_string(),
            args: vec!["x".to_string()],
            definition: |args| {
                if args.len() != 1 {
                    panic!("sin takes exactly 1 argument");
                }
                match &args[0].simplify().simplify() {
                    // TODO This is just for testing purposes, rewrite this when possible
                    Math::Variable(v) => {
                        if v.is_integer() {
                            return Some(Math::Variable(Variable {
                                value: Decimal::from_f64(v.value.to_f64().unwrap().cos()).unwrap(),
                                suffix: String::from(""),
                                exponent: None,
                            }));
                        }
                        return None;
                    }

                    _ => {}
                }
                None
            },
        },
        FunctionDefinition {
            label: "tan".to_string(),
            args: vec!["x".to_string()],
            definition: |args| {
                if args.len() != 1 {
                    panic!("sin takes exactly 1 argument");
                }
                match &args[0].simplify() {
                    // TODO This is just for testing purposes, rewrite this when possible
                    Math::Variable(v) => {
                        if v.is_integer() {
                            return Some(Math::Variable(Variable {
                                value: Decimal::from_f64(v.value.to_f64().unwrap().tan()).unwrap(),
                                suffix: String::from(""),
                                exponent: None,
                            }));
                        }
                        return None;
                    }

                    _ => {}
                }
                None
            },
        },
    ];

    for func in func_defs {
        if func.label == label {
            return Some(func);
        }
    }
    //TODO define a new function on the fly and add to implicit scope
    None
}
