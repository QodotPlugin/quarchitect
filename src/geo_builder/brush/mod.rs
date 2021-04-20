use crate::map::quake::Brush;
use crate::map::quake::Entity;
use crate::TextureInfo;
use crate::Vector3;

mod geometry;

use super::brush_plane;
pub use geometry::Geometry;

pub fn build(textures: &TextureInfo, entity: &Entity, brush: &Brush) -> Geometry {

    // Build planes
    let planes = &brush.planes;
    let plane_geometry: Vec<brush_plane::Geometry> = planes
        .iter()
        .map(|plane| brush_plane::build(textures, entity, planes, plane))
        .collect();

    // Calculate center
    let center = plane_geometry
        .iter()
        .fold(Vector3::new(0.0, 0.0, 0.0), |acc, plane_geometry| {
            acc + plane_geometry.center
        })
        / plane_geometry.len().max(1) as f32;

    Geometry::new(center, plane_geometry)
}
