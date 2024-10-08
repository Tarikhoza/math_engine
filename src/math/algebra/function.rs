use crate::castable::Castable;
use crate::definitions::syscall_functions::find_function_definition;
use crate::math::algebra::exponentable::Exponentable;
use crate::math::algebra::operations::Operations;
use crate::math::algebra::variable::Variable;
use crate::math::simplifiable::Simplifiable;
use crate::math::Math;
use crate::parser::Parsable;

use rust_decimal_macros::dec;

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionalDefinition {
    pub label: String,
    pub args: Vec<String>,
    pub definition: fn(Vec<Math>) -> Option<Math>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MappingDefinition {
    pub label: String,
    pub args: Vec<String>,
    pub definition: Box<Math>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FunctionDefinition {
    MappingDefinition(MappingDefinition),
    FunctionalDefinition(FunctionalDefinition),
}

impl FunctionDefinition {
    fn apply_definition(&self, args: Vec<Math>) -> Option<Math> {
        if args.len() != self.args().len() {
            panic!(
                "\n Expected {} arguments for function {}, got {} arguments ({})\n",
                self.args().len(),
                self.to_tex(),
                args.len(),
                args.iter()
                    .map(|m| m.to_tex())
                    .collect::<Vec<String>>()
                    .join(", ")
            )
        }
        match self {
            FunctionDefinition::MappingDefinition(f) => {
                let mut new_math = *f.definition.clone();
                for (suffix, math) in f.args.iter().zip(args.iter()) {
                    new_math = new_math.substitute(suffix, math.clone());
                }
                return Some(new_math);
            }
            FunctionDefinition::FunctionalDefinition(f) => {
                let sim = args
                    .clone()
                    .iter_mut()
                    .map(|a| {
                        if let Math::Function(Function::FunctionInstance(i)) = a {
                            println!("Def{:#?}", i.definition);
                            i.definition = find_function_definition(&i.label);
                            println!("Def{:#?}", i);
                            return i.apply_definition();
                        } else {
                            return a.clone();
                        }
                    })
                    .collect::<Vec<Math>>();
                (f.definition)(sim)
            }
        }
    }

    pub fn label(&self) -> String {
        match self {
            FunctionDefinition::FunctionalDefinition(f) => f.label.clone(),
            FunctionDefinition::MappingDefinition(f) => f.label.clone(),
        }
    }
    pub fn args(&self) -> Vec<Math> {
        match self {
            FunctionDefinition::FunctionalDefinition(f) => {
                let mut args = Vec::new();
                for i in f.args.iter() {
                    args.push(Math::Variable(Variable {
                        suffix: i.clone(),
                        value: dec!(0),
                        exponent: None,
                    }));
                }
                return args;
            }
            FunctionDefinition::MappingDefinition(f) => {
                let mut args = Vec::new();
                for i in f.args.iter() {
                    args.push(Math::Variable(Variable {
                        suffix: i.clone(),
                        value: dec!(0),
                        exponent: None,
                    }));
                }
                return args;
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionInstance {
    pub label: String,
    pub args: Vec<Math>,
    pub definition: Option<FunctionDefinition>,
    pub exponent: Option<Box<Math>>,
}

impl FunctionInstance {
    pub fn apply_definition(&self) -> Math {
        if let Some(definition) = &self.definition {
            if let Some(result) = definition.apply_definition(self.args.clone()) {
                if let Some(exp) = &self.exponent {
                    return result.as_braces().set_exponent(*exp.clone());
                }
                return result;
            }
        } else {
            panic!("Tried to execute function without definition {:#?}", self);
        }
        Math::Function(Function::FunctionInstance(self.clone()))
    }
    pub fn new(definition: FunctionDefinition, args: Vec<Math>) -> FunctionInstance {
        FunctionInstance {
            label: definition.label(),
            args,
            definition: Some(definition),
            exponent: None,
        }
    }
    pub fn new_with_label(label: String, args: Vec<Math>) -> FunctionInstance {
        FunctionInstance {
            label,
            args,
            definition: None,
            exponent: None,
        }
    }
    pub fn label(&self) -> String {
        return self.label.clone();
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Function {
    FunctionDefinition(FunctionDefinition),
    FunctionInstance(FunctionInstance),
}

impl Function {
    pub fn label(&self) -> String {
        match self {
            Function::FunctionDefinition(f) => f.label(),
            Function::FunctionInstance(f) => f.label(),
        }
    }
    pub fn args(&self) -> Vec<Math> {
        match self {
            Function::FunctionDefinition(f) => f.args(),
            Function::FunctionInstance(f) => f.args.clone(),
        }
    }
    pub fn set_definition(&mut self, definition: Option<FunctionDefinition>) {
        match self {
            Function::FunctionDefinition(f) => *f = definition.unwrap(),
            Function::FunctionInstance(f) => f.definition = definition,
        }
    }
}
