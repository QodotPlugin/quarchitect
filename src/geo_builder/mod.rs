use crate::map::quake::Entity;
use crate::TextureInfo;

pub mod brush;
pub mod brush_plane;
pub mod entity;

pub fn run(textures: &TextureInfo, entities: &[Entity]) -> Vec<entity::Geometry> {
    println!("Running geo builder");
    entities
        .iter()
        .map(|entity| entity::build(textures, entity))
        .collect()
}
