use super::Brush;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Entity {
    pub properties: HashMap<String, String>,
    pub brushes: Vec<Brush>,
}

impl Entity {
    pub fn new() -> Entity {
        let properties = HashMap::new();
        let brushes = Vec::new();
        Entity {
            properties,
            brushes,
        }
    }

    pub fn get_property(&self, name: &str) -> Option<&str> {
        if let Some(s) = self.properties.get(&String::from(name)) {
            return Some(&s[..]);
        }
        None
    }
}
