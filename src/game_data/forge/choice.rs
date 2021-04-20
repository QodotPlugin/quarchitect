use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, PartialEq)]
pub enum ChoiceData {
    Integer(i32),
    Float(f32),
    String(String),
}

impl Display for ChoiceData {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            ChoiceData::Integer(integer) => integer.fmt(f),
            ChoiceData::Float(float) => float.fmt(f),
            ChoiceData::String(string) => string.fmt(f),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Choice {
    pub name: String,
    pub value: ChoiceData,
}

impl Choice {
    pub fn integer(name: &str, value: i32) -> Choice {
        let name = name.to_string();
        let value = ChoiceData::Integer(value);
        Choice { name, value }
    }

    pub fn float(name: &str, value: f32) -> Choice {
        let name = name.to_string();
        let value = ChoiceData::Float(value);
        Choice { name, value }
    }

    pub fn string(name: &str, value: &str) -> Choice {
        let name = name.to_string();
        let value = ChoiceData::String(value.to_string());
        Choice { name, value }
    }
}

impl Default for Choice {
    fn default() -> Self {
        let name = String::new();
        let value = ChoiceData::Integer(0);

        Choice { name, value }
    }
}

impl ToString for Choice {
    fn to_string(&self) -> String {
        let value = match &self.value {
            ChoiceData::Integer(integer) => format!("{}", integer),
            ChoiceData::Float(float) => format!("{}", float),
            ChoiceData::String(string) => format!("\"{}\"", string),
        };
        std::format!("{} : \"{}\"", value, self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integer_to_string() {
        let comp_str = "6 : \"foo\"";
        let base_string = Choice::integer("foo", 6).to_string();

        assert!(
            base_string.as_str() == comp_str,
            "Choice string \"{}\" != \"{}\"",
            base_string,
            comp_str
        );
    }

    #[test]
    fn float_to_string() {
        let comp_str = "12.5 : \"foo\"";
        let base_string = Choice::float("foo", 12.5).to_string();

        assert!(
            base_string.as_str() == comp_str,
            "Choice string \"{}\" != \"{}\"",
            base_string,
            comp_str
        );
    }

    #[test]
    fn string_to_string() {
        let comp_str = "\"bar\" : \"foo\"";
        let base_string = Choice::string("foo", "bar").to_string();

        assert!(
            base_string.as_str() == comp_str,
            "Choice string \"{}\" != \"{}\"",
            base_string,
            comp_str
        );
    }
}
