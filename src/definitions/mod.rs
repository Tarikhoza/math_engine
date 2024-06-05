use crate::{
    math::{
        algebra::function::{Function, FunctionDefinition},
        Math,
    },
    scope::{Scope, ScopeContent},
};

pub mod math_functions;
pub mod syscall_functions;

pub fn find_function_definition(label: &str, current_scope: &Scope) -> Option<FunctionDefinition> {
    if let Some(definition) = math_functions::find_function_definition(label) {
        return Some(definition);
    } else if let Some(definition) = syscall_functions::find_function_definition(label) {
        return Some(definition);
    }
    for content in &current_scope.content {
        if let ScopeContent::Math(Math::Function(Function::FunctionDefinition(
            function_definition,
        ))) = content
        {
            if function_definition.label() == label {
                return Some(function_definition.clone());
            }
        }
    }
    None
}
