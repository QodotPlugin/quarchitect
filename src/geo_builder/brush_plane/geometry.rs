use crate::Vector3;
use super::Vertex;

#[derive(Debug, Clone)]
pub struct Geometry {
    pub center: Vector3,
    pub vertices: Vec<Vertex>,
    pub indices: Vec<usize>,
    pub texture: Option<String>,
}

impl Geometry {
    pub fn new(center: Vector3, vertices: Vec<Vertex>, indices: Vec<usize>, texture:Option<String>) -> Geometry {
        Geometry { center, vertices, indices, texture }
    }
}
