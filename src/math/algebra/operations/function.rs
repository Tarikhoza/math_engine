use crate::math::{
    algebra::function::{Function, FunctionDefinition, FunctionInstance},
    Math, Simplifiable,
};

impl Simplifiable for Function {
    fn simplify(&self) -> Math {
        match self {
            Function::FunctionDefinition(f) => match f {
                FunctionDefinition::MappingDefinition(f) => Math::Function(
                    Function::FunctionDefinition(FunctionDefinition::MappingDefinition(f.clone())),
                ),
                FunctionDefinition::FunctionalDefinition(f) => {
                    Math::Function(Function::FunctionDefinition(
                        FunctionDefinition::FunctionalDefinition(f.clone()),
                    ))
                }
            },
            Function::FunctionInstance(f) => f.simplify(),
        }
    }
}

impl Simplifiable for FunctionInstance {
    fn simplify(&self) -> Math {
        self.apply_definition()
    }
}
