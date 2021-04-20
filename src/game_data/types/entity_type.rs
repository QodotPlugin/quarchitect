#[derive(Debug, Clone)]
pub enum EntityType {
    Placeholder,
    Class(String),
    Prefab(String),
}

impl EntityType {
    pub fn class(class_name: &str) -> EntityType {
        EntityType::Class(class_name.into())
    }

    pub fn prefab(prefab_name: &str) -> EntityType {
        EntityType::Prefab(prefab_name.into())
    }
}

impl Into<i64> for EntityType {
    fn into(self) -> i64 {
        match self {
            EntityType::Placeholder => 0,
            EntityType::Class(_) => 1,
            EntityType::Prefab(_) => 2,
        }
    }
}
