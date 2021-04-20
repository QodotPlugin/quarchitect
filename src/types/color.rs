#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Color {
        Color { r, g, b }
    }
}

impl Default for Color {
    fn default() -> Self {
        let r = 1.0;
        let g = 1.0;
        let b = 1.0;
        Color { r, g, b }
    }
}

impl ToString for Color {
    fn to_string(&self) -> String {
        format!(
            "{} {} {}",
            (self.r * 255.0) as i32,
            (self.g * 255.0) as i32,
            (self.b * 255.0) as i32
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn to_string() {
        let comp_str = "51 102 153";
        let color_string = Color::new(0.2, 0.4, 0.6).to_string();
        assert!(
            color_string.as_str() == comp_str,
            std::format!("Color string \"{}\" != \"{}\"", color_string, comp_str)
        )
    }
}
