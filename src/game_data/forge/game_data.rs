use super::Entity;

#[derive(Debug, Clone)]
pub struct GameData {
    pub name: String,
    pub includes: Vec<String>,
    pub definitions: Vec<Entity>,
}

impl GameData {
    pub fn new(name: String, includes: Vec<String>, definitions: Vec<Entity>) -> GameData {
        GameData {
            name,
            includes,
            definitions,
        }
    }

    pub fn save(&self, file: String) -> std::io::Result<()> {
        std::fs::write(file, self.to_string())?;
        Ok(())
    }
}

impl Default for GameData {
    fn default() -> Self {
        let name = String::new();
        let includes = Vec::new();
        let definitions = Vec::new();

        GameData {
            name,
            includes,
            definitions,
        }
    }
}

impl ToString for GameData {
    fn to_string(&self) -> String {
        let entities = self.definitions.iter().fold(String::new(), |acc, next| {
            acc + &format!("{}\n\n", next.to_string())
        });

        let includes = self.includes.iter().fold(String::new(), |acc, next| {
            acc + &format!("@include \"{}\"\n", next)
        });

        format!("// {}\n\n{}\n{}", self.name, includes, entities)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn to_string() {
        use crate::game_data::forge::{
            Choice, ClassType, Entity, GameData, Metadata, Property, PropertyData,
        };
        use crate::{Color, Vector3};

        const TEST_GAME_DATA_STRING: &str = include_str!("test_data/game_data.fgd");

        let point_entity = Entity::new(
            ClassType::PointClass,
            vec![
                Metadata::Base(vec!["Origin".into()]),
                Metadata::Color(Color::new(0.5, 0.5, 0.5)),
                Metadata::Size(Vector3::new(-4.0, -4.0, -4.0), Vector3::new(4.0, 4.0, 4.0)),
            ],
            "point_class",
            "Point Class",
            vec![
                Property::new(
                    "choices_property",
                    "Choices Property",
                    "This is a choices property",
                    PropertyData::Choices(
                        vec![
                            Choice::float("foo", 1.234),
                            Choice::float("bar", 2.567),
                            Choice::float("baz", 3.891),
                        ],
                        1,
                    ),
                ),
                Property::new(
                    "flags_property",
                    "Flags Property",
                    "This is a flags property",
                    PropertyData::Flags(vec!["foo".into(), "bar".into(), "baz".into()], 1 | 2),
                ),
            ],
        );

        let solid_entity = Entity::new(
            ClassType::SolidClass,
            vec![
                Metadata::Base(vec!["Origin".into()]),
                Metadata::Color(Color::new(0.5, 0.5, 0.5)),
                Metadata::Size(Vector3::new(-4.0, -4.0, -4.0), Vector3::new(4.0, 4.0, 4.0)),
            ],
            "solid_class",
            "Solid Class",
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
        );

        let game_data = GameData::new(
            "Test Game Data".to_string(),
            vec!["base.fgd".into(), "other.fgd".into()],
            vec![point_entity, solid_entity],
        );

        let base_string = game_data.to_string();

        assert!(
            base_string.as_str() == TEST_GAME_DATA_STRING,
            "Game data string\n\"{:?}\"\n!=\n\"{:?}\"",
            base_string,
            TEST_GAME_DATA_STRING
        );
    }
}
