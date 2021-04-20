use super::Color;
use super::Vector2;
use super::Vector3;

#[derive(Debug, Clone)]
pub struct Vertex {
    pub vertex: Vector3,
    pub normal: Vector3,
    pub tangent: (Vector3, f32),
    pub uv: Option<Vector2>,
    pub color: Option<Color>,
}

impl Vertex {
    pub fn new(
        vertex: Vector3,
        normal: Vector3,
        tangent: (Vector3, f32),
        uv: Option<Vector2>,
        color: Option<Color>,
    ) -> Vertex {
        Vertex {
            vertex,
            normal,
            tangent,
            uv,
            color,
        }
    }
}
