use crate::geo_builder;
use crate::map;
use std::collections::HashMap;

pub fn run(
    entity_data: Vec<(map::quake::Entity, geo_builder::entity::Geometry)>,
) -> (
    Vec<(map::quake::Entity, geo_builder::entity::Geometry)>,
    HashMap<String, Vec<geo_builder::brush::Geometry>>,
) {
    let mut worldspawn_layers: HashMap<String, Vec<geo_builder::brush::Geometry>> = HashMap::new();

    let (mut worldspawn_entity_data, entity_data): (
        Vec<(map::quake::Entity, geo_builder::entity::Geometry)>,
        Vec<(map::quake::Entity, geo_builder::entity::Geometry)>,
    ) = entity_data
        .into_iter()
        .partition(|(entity, _)| match entity.get_property("classname") {
            Some(class_name) => class_name == "worldspawn",
            None => false,
        });

    {
        assert!(
            worldspawn_entity_data.len() == 1,
            "Worldspawn element not a singleton"
        );

        let (worldspawn_entity, mut worldspawn_geometry): (
            map::quake::Entity,
            geo_builder::entity::Geometry,
        ) = worldspawn_entity_data.pop().unwrap();

        let worldspawn_brush_geo = worldspawn_geometry.brush_geometry;

        let worldspawn_layer_textures = vec!["*water0", "*slime0"];

        let mut worldspawn_brush_geo = worldspawn_brush_geo;
        for layer_texture in worldspawn_layer_textures {
            let (layer_geometry, worldspawn_geometry) = worldspawn_brush_geo
                .into_iter()
                .partition(brush_geometry_by_layer(layer_texture));

            worldspawn_layers.insert(layer_texture.into(), layer_geometry);
            worldspawn_brush_geo = worldspawn_geometry;
        }

        println!(
            "Worldspawn Brush Geo: {:?}, Worldspawn Layer Geo: {:?}",
            worldspawn_brush_geo.len(),
            worldspawn_layers.len()
        );

        worldspawn_geometry.brush_geometry = worldspawn_brush_geo;
        worldspawn_entity_data.insert(0, (worldspawn_entity, worldspawn_geometry));
    }

    let entity_data = worldspawn_entity_data
        .into_iter()
        .chain(entity_data.into_iter())
        .collect();

    (entity_data, worldspawn_layers)
}

fn brush_geometry_by_layer(layer: &str) -> impl FnMut(&geo_builder::brush::Geometry) -> bool + '_ {
    move |brush_geometry: &geo_builder::brush::Geometry| {
        brush_geometry
            .plane_geometry
            .iter()
            .all(|plane_geometry| match &plane_geometry.texture {
                Some(texture) => texture.as_str() == layer,
                None => false,
            })
    }
}
