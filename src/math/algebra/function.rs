use crate::castable::Castable;
use crate::math::algebra::exponentable::Exponentable;
use crate::math::simplifiable::Simplifiable;
use crate::math::Math;

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDefinition {
    pub label: String,
    pub args: Vec<String>,
    pub definition: fn(Vec<Math>) -> Option<Math>,
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
            if let Some(result) = (definition.definition)(self.args.clone()) {
                if let Some(exp) = &self.exponent {
                    return result.as_braces().set_exponent(*exp.clone());
                }
                return result;
            }
        }
        Math::Function(Function::FunctionInstance(self.clone()))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Function {
    FunctionDefinition(FunctionDefinition),
    FunctionInstance(FunctionInstance),
}
