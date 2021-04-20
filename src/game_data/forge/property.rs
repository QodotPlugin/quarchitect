use super::Choice;
use crate::{Color, Vector3};

#[derive(Debug, Clone, PartialEq)]
pub enum PropertyData {
    Integer(i32),
    Float(f32),
    Vector3(Vector3),
    String(String),
    Color(Color),
    Choices(Vec<Choice>, i32),
    Flags(Vec<String>, i32),
    TargetSource,
    TargetDestination,
}

#[derive(Debug, Clone)]
pub struct Property {
    pub name: String,
    pub short_description: String,
    pub long_description: String,
    pub data: PropertyData,
}

impl Property {
    pub fn new(
        name: &str,
        short_description: &str,
        long_description: &str,
        data: PropertyData,
    ) -> Property {
        let name = name.into();
        let short_description = short_description.into();
        let long_description = long_description.into();
        Property {
            name,
            short_description,
            long_description,
            data,
        }
    }
}

impl Default for Property {
    fn default() -> Self {
        let name = String::new();
        let short_description = String::new();
        let long_description = String::new();
        let data = PropertyData::Integer(0);
        Property {
            name,
            short_description,
            long_description,
            data,
        }
    }
}

impl ToString for Property {
    fn to_string(&self) -> String {
        match &self.data {
            PropertyData::Integer(integer) => format!(
                "{}(integer) : \"{}\" : {} : \"{}\"",
                self.name, self.short_description, integer, self.long_description
            ),
            PropertyData::Float(float) => format!(
                "{}(float) : \"{}\" : {} : \"{}\"",
                self.name, self.short_description, float, self.long_description
            ),
            PropertyData::Vector3(vector3) => format!(
                "{}(string) : \"{}\" : \"{} {} {}\" : \"{}\"",
                self.name,
                self.short_description,
                vector3.x(),
                vector3.y(),
                vector3.z(),
                self.long_description
            ),
            PropertyData::String(string) => format!(
                "{}(string) : \"{}\" : \"{}\" : \"{}\"",
                self.name, self.short_description, string, self.long_description
            ),
            PropertyData::Color(color) => format!(
                "{}(color255) : \"{}\" : \"{}\" : \"{}\"",
                self.name,
                self.short_description,
                color.to_string(),
                self.long_description
            ),
            PropertyData::Choices(choices, default) => {
                let choices_string = choices.iter().fold("".to_string(), |acc, next| {
                    acc + &format!("\t\t{}\n", next.to_string())
                });

                format!(
                    "{}(choices) : \"{}\" : \"{}\" =\n\t[\n{}\t]",
                    self.name,
                    self.short_description,
                    choices[*default as usize].value.to_string(),
                    choices_string
                )
            }
            PropertyData::Flags(flags, default) => {
                let flags_string =
                    flags
                        .iter()
                        .enumerate()
                        .fold(String::new(), |acc, (i, next)| {
                            let bit = 2_i32.pow(i as u32);
                            acc + &format!(
                                "\t\t{} : \"{}\" : {}\n",
                                bit,
                                next,
                                if *default & bit > 0 { 1 } else { 0 }
                            )
                        });
                format!("{}(flags) =\n\t[\n{}\t]", self.name, flags_string)
            }
            PropertyData::TargetSource => format!(
                "{}(target_source) : \"{}\" : : \"{}\"",
                self.name, self.short_description, self.long_description
            ),
            PropertyData::TargetDestination => format!(
                "{}(target_destination) : \"{}\" : : \"{}\"",
                self.name, self.short_description, self.long_description
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn integer_to_string() {
        let comp_str = "int_property(integer) : \"Integer\" : 10 : \"Integer Property\"";
        let base_string = Property::new(
            "int_property",
            "Integer",
            "Integer Property",
            PropertyData::Integer(10),
        )
        .to_string();

        assert!(
            base_string.as_str() == comp_str,
            "Integer string \"{}\" != \"{}\"",
            base_string,
            comp_str
        );
    }

    #[test]
    fn float_to_string() {
        let comp_str = "float_property(float) : \"Float\" : 6.282 : \"Float Property\"";
        let base_string = Property::new(
            "float_property",
            "Float",
            "Float Property",
            PropertyData::Float(6.282),
        )
        .to_string();

        assert!(
            base_string.as_str() == comp_str,
            "Float string \"{}\" != \"{}\"",
            base_string,
            comp_str
        );
    }

    #[test]
    fn vector3_to_string() {
        let comp_str =
            "vector3_property(string) : \"Vector3\" : \"10.5 20.3 30.7\" : \"Vector3 Property\"";
        let base_string = Property::new(
            "vector3_property",
            "Vector3",
            "Vector3 Property",
            PropertyData::Vector3(Vector3::new(10.5, 20.3, 30.7)),
        )
        .to_string();

        assert!(
            base_string.as_str() == comp_str,
            "Vector3 string \"{}\" != \"{}\"",
            base_string,
            comp_str
        );
    }

    #[test]
    fn string_to_string() {
        let comp_str =
        "string_property(string) : \"String\" : \"foo bar baz decafisbad\" : \"String Property\"";
        let base_string = Property::new(
            "string_property",
            "String",
            "String Property",
            PropertyData::String("foo bar baz decafisbad".into()),
        )
        .to_string();

        assert!(
            base_string.as_str() == comp_str,
            "String string \"{}\" != \"{}\"",
            base_string,
            comp_str
        );
    }

    #[test]
    fn color_to_string() {
        let comp_str = "color_property(color255) : \"Color\" : \"51 102 153\" : \"Color Property\"";
        let base_string = Property::new(
            "color_property",
            "Color",
            "Color Property",
            PropertyData::Color(Color::new(0.2, 0.4, 0.6)),
        )
        .to_string();

        assert!(
            base_string.as_str() == comp_str,
            "Color string \"{}\" != \"{}\"",
            base_string,
            comp_str
        );
    }

    #[test]
    fn choices_to_string() {
        let comp_str =
        "choices_property(choices) : \"Choices\" : \"foo\" =\n\t[\n\t\t\"foo\" : \"Foo\"\n\t\t\"bar\" : \"Bar\"\n\t\t\"baz\" : \"Baz\"\n\t]";
        let base_string = Property::new(
            "choices_property",
            "Choices",
            "Choices Property",
            PropertyData::Choices(
                vec![
                    Choice::string("Foo", "foo"),
                    Choice::string("Bar", "bar"),
                    Choice::string("Baz", "baz"),
                ],
                0,
            ),
        )
        .to_string();

        assert!(
            base_string.as_str() == comp_str,
            "Choices string\n\"{:?}\"\n!=\n\"{:?}\"",
            base_string,
            comp_str
        );
    }

    #[test]
    fn flags_to_string() {
        let comp_str =
        "flags_property(flags) =\n\t[\n\t\t1 : \"foo\" : 1\n\t\t2 : \"bar\" : 0\n\t\t4 : \"baz\" : 1\n\t]";
        let base_string = Property::new(
            "flags_property",
            "Flags",
            "Flags Property",
            PropertyData::Flags(vec!["foo".into(), "bar".into(), "baz".into()], 1 | 4),
        )
        .to_string();

        assert!(
            base_string.as_str() == comp_str,
            "Flags string\n\"{:?}\"\n!=\n\"{:?}\"",
            base_string,
            comp_str
        );
    }

    #[test]
    fn target_source_to_string() {
        let comp_str =
        "target_source_property(target_source) : \"Target Source\" : : \"Target Source Property\"";
        let base_string = Property::new(
            "target_source_property",
            "Target Source",
            "Target Source Property",
            PropertyData::TargetSource,
        )
        .to_string();

        assert!(
            base_string.as_str() == comp_str,
            "Target source string \"{}\" != \"{}\"",
            base_string,
            comp_str
        );
    }

    #[test]
    fn target_destination_to_string() {
        let comp_str = "target_destination_property(target_destination) : \"Target Destination\" : : \"Target Destination Property\"";
        let base_string = Property::new(
            "target_destination_property",
            "Target Destination",
            "Target Destination Property",
            PropertyData::TargetDestination,
        )
        .to_string();

        assert!(
            base_string.as_str() == comp_str,
            "Target destination string \"{}\" != \"{}\"",
            base_string,
            comp_str
        );
    }
}
