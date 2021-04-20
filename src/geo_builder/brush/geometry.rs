use crate::Vector3;
use super::brush_plane;

#[derive(Debug, Clone)]
pub struct Geometry {
    pub center: Vector3,
    pub plane_geometry: Vec<brush_plane::Geometry>,
}

impl<'a> Geometry {
    pub fn new(center: Vector3, plane_geometry: Vec<brush_plane::Geometry>) -> Geometry {
        Geometry {
            center,
            plane_geometry
        }
    }
}
