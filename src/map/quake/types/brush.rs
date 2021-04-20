use super::BrushPlane;

#[derive(Debug, Default)]
pub struct Brush {
    pub planes: Vec<BrushPlane>,
}

impl Brush {
    pub fn new() -> Brush {
        let planes: Vec<BrushPlane> = Vec::new();
        Brush { planes }
    }
}