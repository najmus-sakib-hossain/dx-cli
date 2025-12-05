use std::collections::HashMap;

use crate::engine::Generator;

#[derive(Default)]
pub struct TemplateRegistry {
    generators: HashMap<String, Box<dyn Generator>>, 
}

impl TemplateRegistry {
    pub fn new() -> Self {
        Self {
            generators: HashMap::new(),
        }
    }

    pub fn register(&mut self, generator: Box<dyn Generator>) {
        self.generators.insert(generator.name().to_string(), generator);
    }

    pub fn list(&self) -> Vec<String> {
        self.generators.keys().cloned().collect()
    }
}
