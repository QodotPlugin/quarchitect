use std::collections::HashMap;

use crate::Color;
use crate::Vector3;

#[derive(Debug, Clone)]
pub enum Property {
    Integer(i32),
    Float(f32),
    Vector3(Vector3),
    String(String),
    Color(Color),
    Choices(i32),
    Flags(i32),
    TargetSource,
    TargetDestination
}

#[derive(Debug, Clone)]
pub struct Properties(pub HashMap<String, Property>);

impl Properties {
    pub fn new(properties: HashMap<String, Property>) -> Properties {
        Properties(properties)
    }
}

impl Default for Properties {
    fn default() -> Properties {
        Properties(HashMap::new())
    }
}
