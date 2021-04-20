use crate::Vector3;
use crate::map::quake::Entity;
use crate::map::quake::BrushPlane;

const ONE_DEGREE: f32 = 0.017_453_3;

pub fn vertex_normal(entity: &Entity, p0: &BrushPlane, p1: &BrushPlane, p2: &BrushPlane) -> Vector3 {
    if let Some("1") = entity.get_property("_phong") {
        return phong_normal(p0, p1, p2, entity.get_property("_phong_angle"));
    }

    p0.normal()
}

fn phong_normal(
    p0: &BrushPlane,
    p1: &BrushPlane,
    p2: &BrushPlane,
    phong_angle: Option<&str>,
) -> Vector3 {
    if let Some(phong_angle) = phong_angle {
        if let Ok(phong_angle) = phong_angle.parse::<f32>() {
            let threshold = ((phong_angle + 0.01) * ONE_DEGREE).cos();
            let mut normal = p0.normal();
            if p0.normal().dot(p1.normal()) > threshold {
                normal += p1.normal()
            }
            if p0.normal().dot(p2.normal()) > threshold {
                normal += p2.normal()
            }
            return normal.normalize();
        }
    }

    (p0.normal() + p1.normal() + p2.normal()).normalize()
}