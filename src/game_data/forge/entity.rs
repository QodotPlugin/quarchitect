use super::{Metadata, Property};

#[derive(Debug, Copy, Clone)]
pub enum ClassType {
    BaseClass,
    PointClass,
    SolidClass,
}

impl Into<i64> for ClassType {
    fn into(self) -> i64 {
        match self {
            ClassType::BaseClass => 0,
            ClassType::PointClass => 1,
            ClassType::SolidClass => 2,
        }
    }
}

impl From<i64> for ClassType {
    fn from(val: i64) -> Self {
        match val {
            0 => ClassType::BaseClass,
            1 => ClassType::PointClass,
            2 => ClassType::SolidClass,
            _ => panic!("Unexpected class type"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Entity {
    pub class_type: ClassType,
    pub metadata: Vec<Metadata>,
    pub class_name: String,
    pub description: String,
    pub properties: Vec<Property>,
}

impl std::hash::Hash for Entity {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.class_name.hash(state);
    }
}

impl PartialEq for Entity {
    fn eq(&self, other: &Entity) -> bool {
        self.class_name == other.class_name
    }
}

impl Entity {
    pub fn new(
        class_type: ClassType,
        metadata: Vec<Metadata>,
        class_name: &str,
        description: &str,
        properties: Vec<Property>,
    ) -> Entity {
        let class_name = class_name.into();
        let description = description.into();
        Entity {
            class_type,
            metadata,
            class_name,
            description,
            properties,
        }
    }
}

impl Eq for Entity {}

impl Default for Entity {
    fn default() -> Self {
        let class_type = ClassType::BaseClass;
        let metadata = Vec::new();
        let class_name = String::new();
        let description = String::new();
        let properties = Vec::new();

        Entity {
            class_type,
            metadata,
            class_name,
            description,
            properties,
        }
    }
}

impl ToString for Entity {
    fn to_string(&self) -> String {
        let class = match self.class_type {
            ClassType::BaseClass => "@BaseClass",
            ClassType::PointClass => "@PointClass",
            ClassType::SolidClass => "@SolidClass",
        };

        let metadata = self.metadata.iter().fold(String::new(), |acc, next| {
            acc + &format!("{} ", next.to_string())
        });

        let properties = self.properties.iter().fold(String::new(), |acc, next| {
            acc + &format!("\t{}\n", next.to_string())
        });

        format!(
            "{} {}= {} : \"{}\"\n[\n{}]",
            class, metadata, self.class_name, self.description, properties
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_data::forge::PropertyData;

    #[test]
    fn to_string() {
        use crate::{Color, Vector3};

        const TEST_ENTITY_STRING: &str = include_str!("test_data/entity.fgd");

        let base_string = Entity::new(
            ClassType::SolidClass,
            vec![
                Metadata::Base(vec!["Origin".into()]),
                Metadata::Color(Color::new(0.5, 0.5, 0.5)),
                Metadata::Size(Vector3::new(-4.0, -4.0, -4.0), Vector3::new(4.0, 4.0, 4.0)),
            ],
            "example_class",
            "Example Class",
            vec![
                Property::new(
                    "integer_property",
                    "Integer Property",
                    "This is an integer property",
                    PropertyData::Integer(1234),
                ),
                Property::new(
                    "float_property",
                    "Float Property",
                    "This is a float property",
                    PropertyData::Float(6.282),
                ),
            ],
        )
        .to_string();

        assert!(
            base_string.as_str() == TEST_ENTITY_STRING,
            "Entity string\n\"{:?}\"\n!=\n\"{:?}\"",
            base_string,
            TEST_ENTITY_STRING
        );
    }
}
