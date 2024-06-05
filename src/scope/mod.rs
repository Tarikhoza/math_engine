use crate::definitions::find_function_definition;
use crate::logging::env_info;
use crate::math::algebra::function::{Function, FunctionDefinition, FunctionInstance};
use crate::math::simplifiable::Simplifiable;
use crate::math::Math;

#[derive(Clone, Debug)]
pub enum ScopeContent {
    Scope(Scope),
    Math(Math),
    //TODO Write logic parser
    //Logic(Logic)...
}

#[derive(Clone, Debug)]
pub struct Scope {
    pub content: Vec<ScopeContent>,
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

    pub fn add_and_inject(&mut self, content: &Math) {
        match &content {
            // TODO Inject Other Math, not just functions
            // TODO Solve Recursive function injection(function in a function, sin(sin(3)) does not work yet)
            Math::Function(f) => {
                // Find definition in scope
                for content in &self.content {
                    if let ScopeContent::Math(Math::Function(Function::FunctionDefinition(
                        function_definition,
                    ))) = content
                    {
                        if f.label() == function_definition.label() {
                            // Inject definition to function
                            env_info(
                                "scope",
                                format!(
                                    "found definition to function: {:#?} in scope",
                                    function_definition
                                ),
                            );
                            let mut function_instance = FunctionInstance::new(
                                function_definition.clone(),
                                f.args().clone(),
                            );
                            self.content.push(ScopeContent::Math(Math::Function(
                                Function::FunctionInstance(function_instance),
                            )));
                            return;
                        }
                    }
                }

                // Find in predefined function library
                // TODO maybe just push all the functions to the scope, or make a standard scope
                if let Some(definition) = find_function_definition(&f.label(), self) {
                    // Inject definition to function
                    env_info(
                        "scope",
                        format!(
                            "found definition to function: {:#?} in math_function_definitions",
                            definition
                        ),
                    );

                    let mut function_instance =
                        FunctionInstance::new(definition.clone(), f.args().clone());
                    function_instance.definition = Some(definition);
                    self.content.push(ScopeContent::Math(Math::Function(
                        Function::FunctionInstance(function_instance),
                    )));
                    return;
                }
            }

            _ => {
                env_info("scope", "did not find any definition".into());
                self.content.push(ScopeContent::Math(content.clone()));
                return;
            }
        }
        env_info("scope", "did not find any definition".into());
        self.content.push(ScopeContent::Math(content.clone()));
    }
    // TODO Think about what is executed and not, how to determine what should be executed
    // Maybe #$some math$ should be executed and $some math$ should just fly around
    // Or sys / inbuild functions like print should be always be executed

    pub fn simplify_last(&self) -> Result<Math, String> {
        if let Some(content) = self.content.last() {
            if let ScopeContent::Math(math) = content {
                return Ok(math.simplify());
            }
        }
        return Err("Could not find last. Maybe file is empty?".to_string());
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
}
