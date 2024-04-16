use crate::definitions::math_functions::find_function_definition as find_math_function_definition;
use crate::definitions::syscall_functions::find_function_definition as find_syscall_function_definition;
use crate::math::algebra::function::{Function, FunctionDefinition, FunctionInstance};
use crate::math::simplifiable::Simplifiable;
use crate::math::Math;

#[derive(Clone, Debug)]
pub enum ScopeContent {
    Scope(Scope),
    Math(Math),
    //Logic(Logic)...
}

#[derive(Clone, Debug)]
pub struct Scope {
    content: Vec<ScopeContent>,
}

impl Scope {
    pub fn new() -> Scope {
        Scope {
            content: Vec::new(),
        }
    }
    pub fn add(&mut self, content: Math) {
        self.content.push(ScopeContent::Math(content));
    }

    pub fn add_and_inject(&mut self, content: Math) {
        match content {
            Math::Function(Function::FunctionInstance(f)) => {
                for content in &self.content {
                    if let ScopeContent::Math(Math::Function(Function::FunctionDefinition(
                        function_definition,
                    ))) = content
                    {
                        if f.label == function_definition.label() {
                            // inject definition to function
                            println!(
                                "found definition to function: {:#?} in scope",
                                function_definition
                            );
                            println!("inject definition to function: {}", f.label);
                            let mut function_instance =
                                FunctionInstance::new(function_definition.clone(), f.args.clone());
                            self.content.push(ScopeContent::Math(Math::Function(
                                Function::FunctionInstance(function_instance),
                            )));
                            return;
                        }
                    }
                }

                if let Some(definition) = find_math_function_definition(&f.label) {
                    // inject definition to function
                    println!(
                        "found definition to function: {:#?} in math_function_definitions",
                        definition
                    );
                    println!("inject definition to function: {}", f.label);

                    let mut function_instance =
                        FunctionInstance::new(definition.clone(), f.args.clone());
                    function_instance.definition = Some(definition);
                    self.content.push(ScopeContent::Math(Math::Function(
                        Function::FunctionInstance(function_instance),
                    )));
                    return;
                }

                if let Some(definition) = find_syscall_function_definition(&f.label) {
                    println!(
                        "found definition to function: {:#?} in syscall_function_definitions",
                        definition
                    );
                    println!("inject definition to function: {}", f.label);
                    let mut function_instance = FunctionInstance::new(definition.clone(), f.args);
                    function_instance.definition = Some(definition);

                    self.content.push(ScopeContent::Math(Math::Function(
                        Function::FunctionInstance(function_instance),
                    )));
                    return;
                }
            }

            Math::Function(Function::FunctionDefinition(f)) => {
                for content in &self.content {
                    if let ScopeContent::Math(Math::Function(Function::FunctionDefinition(
                        function_definition,
                    ))) = content
                    {
                        if f.label() == function_definition.label() {
                            // inject definition to function
                            println!(
                                "found definition to function: {:#?} in scope",
                                function_definition
                            );
                            println!("inject definition to function: {}", f.label());
                            let mut function_instance =
                                FunctionInstance::new(function_definition.clone(), f.args());
                            self.content.push(ScopeContent::Math(Math::Function(
                                Function::FunctionInstance(function_instance),
                            )));
                            return;
                        }
                    }
                }

                if let Some(definition) = find_math_function_definition(&f.label()) {
                    // inject definition to function
                    println!(
                        "found definition to function: {:#?} in math_function_definitions",
                        definition
                    );
                    println!("inject definition to function: {}", f.label());

                    let mut function_instance = FunctionInstance::new(definition.clone(), f.args());
                    function_instance.definition = Some(definition);
                    self.content.push(ScopeContent::Math(Math::Function(
                        Function::FunctionInstance(function_instance),
                    )));
                    return;
                }

                if let Some(definition) = find_syscall_function_definition(&f.label()) {
                    println!(
                        "found definition to function: {:#?} in syscall_function_definitions",
                        definition
                    );
                    println!("inject definition to function: {}", f.label());
                    let mut function_instance = FunctionInstance::new(definition.clone(), f.args());
                    function_instance.definition = Some(definition);

                    self.content.push(ScopeContent::Math(Math::Function(
                        Function::FunctionInstance(function_instance),
                    )));
                    return;
                }
            }

            _ => todo!("add and inject not implemented"),
        }
    }

    pub fn simplify_last(&mut self) -> Result<Math, String> {
        if let Some(content) = self.content.last_mut() {
            if let ScopeContent::Math(math) = content {
                return Ok(math.simplify());
            }
        }
        return Err("could not find last".to_string());
    }

    pub fn merge(&self, other: &Scope) -> Scope {
        Scope {
            content: self
                .content
                .clone()
                .into_iter()
                .chain(other.content.clone())
                .collect(),
        }
    }
    pub fn find_definition(&self, name: &str) -> Option<Math> {
        return None;
    }
}
