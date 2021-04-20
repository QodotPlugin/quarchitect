#[derive(Debug, Copy, Clone, PartialEq)]
pub enum VisualType {
    None,
    Mesh,
}

impl Into<i64> for VisualType {
    fn into(self) -> i64 {
        match self {
            VisualType::None => 0,
            VisualType::Mesh => 1,
        }
    }
}

impl From<i64> for VisualType {
    fn from(i: i64) -> Self {
        match i {
            0 => VisualType::None,
            1 => VisualType::Mesh,
            _ => panic!("Invalid visual type"),
        }
    }
}
