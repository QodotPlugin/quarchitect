use super::brush;
use crate::Vector3;

#[derive(Debug, Clone)]
pub struct Geometry {
    pub center: Vector3,
    pub brush_geometry: Vec<brush::Geometry>,
}

impl Geometry {
    pub fn new(center: Vector3, brush_geometry: Vec<brush::Geometry>) -> Geometry {
        Geometry {
            center,
            brush_geometry,
        }
    }
}
