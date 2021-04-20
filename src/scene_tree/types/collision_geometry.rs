use crate::Vector3;

#[derive(Debug, Clone)]
pub enum CollisionGeometry {
    None,
    Convex(Vec<ConvexCollision>),
    Concave(Vec<ConcaveCollision>),
}

impl CollisionGeometry {
    pub fn convex(shapes: Vec<ConvexCollision>) -> CollisionGeometry {
        CollisionGeometry::Convex(shapes)
    }

    pub fn concave(shapes: Vec<ConcaveCollision>) -> CollisionGeometry {
        CollisionGeometry::Concave(shapes)
    }
}

#[derive(Debug, Clone)]
pub struct ConvexCollision {
    pub center: Vector3,
    pub points: Vec<Vector3>,
}

impl ConvexCollision {
    pub fn new(center: Vector3, points: Vec<Vector3>) -> ConvexCollision {
        ConvexCollision { center, points }
    }
}

#[derive(Debug, Clone)]
pub struct ConcaveCollision {
    pub center: Vector3,
    pub vertices: Vec<Vector3>,
    pub indices: Vec<usize>,
}

impl ConcaveCollision {
    pub fn new(center: Vector3, vertices: Vec<Vector3>, indices: Vec<usize>) -> ConcaveCollision {
        ConcaveCollision {
            center,
            vertices,
            indices,
        }
    }
}
