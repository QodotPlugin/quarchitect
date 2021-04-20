use crate::map::quake::BrushPlane;
use crate::Vector3;

const CMP_EPSILON: f32 = 0.001;

pub fn intersect_brush_planes(
    p0: &BrushPlane,
    p1: &BrushPlane,
    p2: &BrushPlane,
) -> Option<Vector3> {
    let n0 = p0.normal();
    let n1 = p1.normal();
    let n2 = p2.normal();

    let denom = n0.cross(n1).dot(n2);

    if denom < CMP_EPSILON {
        return None;
    }

    Some(
        (n1.cross(n2) * p0.dist() + n2.cross(n0) * p1.dist() + n0.cross(n1) * p2.dist()) / denom,
    )
}

pub fn vertex_in_hull(vertex: Vector3, hull: &[BrushPlane]) -> bool {
    for brush_plane in hull {
        let proj = brush_plane.normal().dot(vertex);
        if proj > brush_plane.dist() && proj - brush_plane.dist() > CMP_EPSILON {
            return false;
        }
    }
    true
}
