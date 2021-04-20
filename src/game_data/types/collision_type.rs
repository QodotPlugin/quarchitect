#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CollisionType {
    None,
    Convex,
    Concave,
}

impl Into<i64> for CollisionType {
    fn into(self) -> i64 {
        match self {
            CollisionType::None => 0,
            CollisionType::Convex => 1,
            CollisionType::Concave => 2,
        }
    }
}

impl From<i64> for CollisionType {
    fn from(i: i64) -> Self {
        match i {
            0 => CollisionType::None,
            1 => CollisionType::Convex,
            2 => CollisionType::Concave,
            _ => panic!("Invalid collision type"),
        }
    }
}
