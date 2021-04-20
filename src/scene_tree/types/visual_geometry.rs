use crate::Color;
use crate::Vector2;
use crate::Vector3;

#[derive(Debug, Clone)]
pub enum VisualGeometry {
    None,
    Mesh(VisualMesh),
}

#[derive(Debug, Clone)]
pub struct VisualMesh {
    pub surfaces: Vec<MeshSurface>,
}

impl VisualMesh {
    pub fn new(surfaces: Vec<MeshSurface>) -> VisualMesh {
        VisualMesh { surfaces }
    }
}

#[derive(Debug, Clone)]
pub struct MeshSurface {
    pub texture: Option<String>,
    pub vertices: Vec<Vector3>,
    pub normals: Vec<Vector3>,
    pub tangents: Vec<(Vector3, f32)>,
    pub uvs: Option<Vec<Vector2>>,
    pub colors: Option<Vec<Color>>,
    pub indices: Vec<usize>,
}

impl MeshSurface {
    pub fn new(
        texture: Option<String>,
        vertices: Vec<Vector3>,
        normals: Vec<Vector3>,
        tangents: Vec<(Vector3, f32)>,
        uvs: Option<Vec<Vector2>>,
        colors: Option<Vec<Color>>,
        indices: Vec<usize>,
    ) -> MeshSurface {
        MeshSurface {
            texture,
            vertices,
            normals,
            tangents,
            uvs,
            colors,
            indices,
        }
    }
}
