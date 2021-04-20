use crate::Color;
use crate::Vector3;

#[derive(Debug, Clone)]
pub enum Metadata {
    Base(Vec<String>),
    Color(Color),
    Size(Vector3, Vector3),
}

impl Metadata {
    pub fn base(base_classes: Vec<String>) -> Metadata {
        Metadata::Base(base_classes)
    }

    pub fn color(color: Color) -> Metadata {
        Metadata::Color(color)
    }

    pub fn size(min: Vector3, max: Vector3) -> Metadata {
        Metadata::Size(min, max)
    }
}

impl From<Metadata> for i32 {
    fn from(metadata: Metadata) -> Self {
        match metadata {
            Metadata::Base(_) => 0,
            Metadata::Color(_) => 1,
            Metadata::Size(_, _) => 2,
        }
    }
}

impl ToString for Metadata {
    fn to_string(&self) -> String {
        match self {
            Metadata::Base(base_classes) => std::format!(
                "base({})",
                base_classes
                    .iter()
                    .fold(String::new(), |acc, next| if acc.is_empty() {
                        acc
                    } else {
                        acc + ", "
                    } + &next)
            ),
            Metadata::Color(color) => std::format!("color({})", color.to_string()),
            Metadata::Size(min, max) => std::format!(
                "size({}, {})",
                format!("{}, {}, {}", min.x(), min.y(), min.z()),
                format!("{}, {}, {}", max.x(), max.y(), max.z())
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_to_string() {
        let comp_str = "base(foo, bar, baz)";
        let base_string =
            Metadata::Base(vec!["foo".into(), "bar".into(), "baz".into()]).to_string();

        assert!(
            base_string.as_str() == comp_str,
            "Base string \"{}\" != \"{}\"",
            base_string,
            comp_str
        );
    }

    #[test]
    fn color_to_string() {
        let comp_str = "color(63 127 191)";
        let base_string = Metadata::Color(Color::new(0.25, 0.5, 0.75)).to_string();

        assert!(
            base_string.as_str() == comp_str,
            "Color string \"{}\" != \"{}\"",
            base_string,
            comp_str
        );
    }

    #[test]
    fn size_to_string() {
        let comp_str = "size(-1, -2, -3, 4, 5, 6)";
        let base_string =
            Metadata::Size(Vector3::new(-1.0, -2.0, -3.0), Vector3::new(4.0, 5.0, 6.0)).to_string();

        assert!(
            base_string.as_str() == comp_str,
            "Size string \"{}\" != \"{}\"",
            base_string,
            comp_str
        );
    }
}
