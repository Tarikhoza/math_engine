use crate::math::Math;
use crate::parser::{Parsable, ParsablePrimitive};
use crate::scope::Scope;
use std::fs;

pub struct File {
    path: String,
    pub scope: Scope,
}

impl File {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
            scope: Scope::new(),
        }
    }

    pub fn open(&mut self) {
        let file_content = fs::read_to_string(&self.path).unwrap();
        for line in file_content.lines() {
            if line.trim().is_empty() {
                continue;
            }
            let math = line.parse_math().unwrap();
            self.scope.add_and_inject(&math);
        }
    }

    pub fn eval_last(&self) -> Result<Math, String> {
        return self.scope.simplify_last();
    }
}
