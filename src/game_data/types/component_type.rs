#[derive(Debug, Clone, PartialEq)]
pub enum ComponentType {
    None,
    Script(String),
}

impl Into<i64> for ComponentType {
    fn into(self) -> i64 {
        match self {
            ComponentType::None => 0,
            ComponentType::Script(_) => 1,
        }
    }
}
