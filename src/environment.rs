use std::collections::HashMap;

use crate::object::Object;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Environment {
    pub store: HashMap<String, Object>,
    pub outer: Option<Box<Self>>
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            store: HashMap::new(),
            outer: None
        }
    }

    pub fn create_child(self) -> Self {
        Environment {
            store: HashMap::new(),
            outer: Some(Box::new(self))
        }
    }

    pub fn set(&mut self, name: String, value: Object) {
        self.store.insert(name, value);
    }

    pub fn get(&self, name: &String) -> Option<&Object> {
        self.store.get(name).or_else(|| self.outer.as_ref().and_then(|outer| outer.get(name)))
    }
}