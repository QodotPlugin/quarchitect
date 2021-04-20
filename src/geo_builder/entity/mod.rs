use crate::map::quake::Entity;
use crate::TextureInfo;
use crate::Vector3;

mod geometry;

use super::brush;
pub use geometry::Geometry;

pub fn build(textures: &TextureInfo, entity: &Entity) -> Geometry {

    // Build brushes
    let brush_geometry: Vec<brush::Geometry> = entity
        .brushes
        .iter()
        .map(|brush| brush::build(textures, entity, brush))
        .collect();

    // Calculate center
    let origin = entity.properties.get("origin");
    let center: Vector3 = match origin {
        Some(origin) => {
            let mut comps = origin.split(' ');
            let x: f32 = comps.next().unwrap_or("0.0").parse().unwrap_or(0.0);
            let y: f32 = comps.next().unwrap_or("0.0").parse().unwrap_or(0.0);
            let z: f32 = comps.next().unwrap_or("0.0").parse().unwrap_or(0.0);
            Vector3::new(x, y, z)
        }
        None => {
            brush_geometry
                .iter()
                .fold(Vector3::new(0.0, 0.0, 0.0), |acc, next| acc + next.center)
                / (brush_geometry.len().max(1) as f32)
        }
    };

    Geometry::new(center, brush_geometry)
}
