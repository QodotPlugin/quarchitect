use std::collections::HashMap;

pub use types::{
    Actor, CollisionGeometry, MeshSurface, SceneTreeNode, SceneTreeType, VisualGeometry,
};

use types::{ConcaveCollision, ConvexCollision, VisualMesh};

use crate::{
    game_data::{
        BrushData, CollisionType, ComponentType, EntityType, GameData, Properties, Property,
        PropertyApplicationType, VisualType, WorldspawnLayer,
    },
    geo_builder::{brush, entity},
    map::quake::Entity,
    Color, TextureBlacklist, Vector2, Vector3, Vertex,
};

mod predicates;
mod types;

pub fn run(
    forge_game_data: &crate::game_data::forge::GameData,
    quarchitect_game_data: &crate::game_data::GameData,
    texture_blacklist: &crate::types::TextureBlacklist,
    entity_data: &[(Entity, entity::Geometry)],
    worldspawn_layers: &HashMap<String, Vec<brush::Geometry>>,
) -> Vec<SceneTreeNode> {
    println!("TODO-2: Implement godot group assignment");
    // Global '_groups' property, comma-separated string of group names to apply during build

    println!("TODO-1: Make TB groups optional");
    // Should they default to on now they're more user-friendly?

    println!("TODO-1: Implement group node type assignment");
    // Introduce a new 'group_entity' point class that takes a brush entity classname as a parameter,
    // and causes its parent group to spawn as an instance of that classname instead of as a func_group

    entity_data
        .iter()
        .take(1)
        .flat_map(build_entity(
            quarchitect_game_data,
            forge_game_data,
            texture_blacklist,
            entity_data,
            Vector3::new(0.0, 0.0, 0.0)
        ))
        .chain(
            worldspawn_layers
                .iter()
                .flat_map(build_worldspawn_layer(quarchitect_game_data)),
        )
        .chain(
            entity_data
                .iter()
                .skip(1)
                .filter(|(entity, _geometry)| entity.properties.get("_tb_group").is_none())
                .flat_map(build_entity(
                    quarchitect_game_data,
                    forge_game_data,
                    texture_blacklist,
                    entity_data,
                    Vector3::new(0.0, 0.0, 0.0)
                )),
        )
        .collect()
}

fn build_entity<'a>(
    quarchitect_game_data: &'a GameData,
    forge_game_data: &'a crate::game_data::forge::GameData,
    texture_blacklist: &'a TextureBlacklist,
    entity_data: &'a [(Entity, entity::Geometry)],
    origin: Vector3
) -> impl Fn(&'a (Entity, entity::Geometry)) -> Option<SceneTreeNode> + 'a {
    move |(entity, entity_geometry): &(Entity, entity::Geometry)| {
        let mut children: Vec<SceneTreeNode> = Vec::new();

        match get_entity_visual_geometry(
            quarchitect_game_data,
            texture_blacklist,
            entity,
            entity_geometry,
        ) {
            VisualGeometry::None => (),
            v => children.push(SceneTreeNode::visual_geometry(entity_geometry.center, v)),
        }

        match get_entity_collision_geometry(quarchitect_game_data, entity, entity_geometry) {
            CollisionGeometry::None => (),
            c => children.push(SceneTreeNode::collision_geometry(entity_geometry.center, c)),
        }

        let tb_id = entity.properties.get("_tb_id");
        let mut child_entities: Vec<SceneTreeNode> = entity_data
            .iter()
            .skip(1)
            .filter(|(child_entity, _geometry)| {
                let child_group = child_entity.properties.get("_tb_group");
                child_group.is_some() && child_group == tb_id
            })
            .flat_map(build_entity(
                quarchitect_game_data,
                forge_game_data,
                texture_blacklist,
                entity_data,
                -entity_geometry.center
            ))
            .collect();

        children.append(&mut child_entities);

        Some(SceneTreeNode::entity(
            entity.properties.get("classname").unwrap().clone(),
            origin + entity_geometry.center,
            get_entity_type(quarchitect_game_data, &entity),
            get_entity_component_class(quarchitect_game_data, &entity),
            get_entity_property_application_type(quarchitect_game_data, &entity),
            get_entity_properties(forge_game_data, &entity),
            children,
        ))
    }
}

fn build_worldspawn_layer<'a>(
    quarchitect_game_data: &'a crate::game_data::GameData,
) -> impl Fn((&String, &Vec<brush::Geometry>)) -> Option<SceneTreeNode> + 'a {
    move |(layer_texture, brush_geometry): (&String, &Vec<brush::Geometry>)| {
        let layer_data: Vec<&WorldspawnLayer> = quarchitect_game_data
            .worldspawn_layers
            .iter()
            .filter(|worldspawn_layer| worldspawn_layer.texture == *layer_texture)
            .collect();

        let mut name: String = String::new();
        let mut actor_type = EntityType::Placeholder;
        let mut component_class = None;
        let property_application_type = PropertyApplicationType::Properties;
        let properties = Properties::default();
        let mut children: Vec<SceneTreeNode> = Vec::new();

        if layer_data.len() == 1 {
            let layer_data: &WorldspawnLayer = layer_data[0];
            name = layer_data.name.clone();
            actor_type = layer_data.entity_type.clone();
            component_class = match &layer_data.component_type {
                ComponentType::Script(script_class) => Some(script_class.clone()),
                ComponentType::None => None,
            };

            let visual_geometry =
                get_worldspawn_layer_visual_geometry(layer_data, layer_texture, brush_geometry);
            match visual_geometry {
                VisualGeometry::None => (),
                _ => children.push(SceneTreeNode::visual_geometry(
                    Vector3::default(),
                    visual_geometry,
                )),
            }

            let collision_geometry =
                get_worldspawn_layer_collision_geometry(layer_data, brush_geometry);
            match collision_geometry {
                CollisionGeometry::None => (),
                _ => children.push(SceneTreeNode::collision_geometry(
                    Vector3::default(),
                    get_worldspawn_layer_collision_geometry(layer_data, brush_geometry),
                )),
            }
        } else {
            println!(
                "Worldspawn layer data for texture {:?} is not a singleton",
                layer_texture
            );
        }

        if children.is_empty() {
            return None;
        }

        Some(SceneTreeNode::entity(
            name,
            Vector3::default(),
            actor_type,
            component_class,
            property_application_type,
            properties,
            children,
        ))
    }
}

fn get_entity_type(
    quarchitect_game_data: &crate::game_data::GameData,
    entity: &Entity,
) -> EntityType {
    let classname = entity.properties.get("classname");
    match classname {
        Some(classname) => {
            let entity = quarchitect_game_data
                .entities
                .iter()
                .find(|entity_definition| &entity_definition.classname == classname);

            match entity {
                Some(entity) => match &entity.data {
                    crate::game_data::EntityData::Point(point_data)
                    | crate::game_data::EntityData::Brush(point_data, _) => {
                        point_data.entity_type.clone()
                    }
                },
                None => EntityType::Placeholder,
            }
        }
        None => EntityType::Placeholder,
    }
}

fn get_entity_component_class(quarchitect_game_data: &GameData, entity: &Entity) -> Option<String> {
    let classname = entity.properties.get("classname");
    match classname {
        Some(classname) => {
            let entity = quarchitect_game_data
                .entities
                .iter()
                .find(|entity_definition| &entity_definition.classname == classname);

            match entity {
                Some(entity) => match &entity.data {
                    crate::game_data::EntityData::Point(point_data)
                    | crate::game_data::EntityData::Brush(point_data, _) => {
                        match &point_data.component_type {
                            ComponentType::Script(component) => Some(component.to_string()),
                            ComponentType::None => None,
                        }
                    }
                },
                None => None,
            }
        }
        None => None,
    }
}

fn get_entity_property_application_type(
    quarchitect_game_data: &GameData,
    entity: &Entity,
) -> PropertyApplicationType {
    let classname = entity.properties.get("classname");
    match classname {
        Some(classname) => {
            let entity = quarchitect_game_data
                .entities
                .iter()
                .find(|entity_definition| &entity_definition.classname == classname);

            match entity {
                Some(entity) => match &entity.data {
                    crate::game_data::EntityData::Point(point_data)
                    | crate::game_data::EntityData::Brush(point_data, _) => {
                        point_data.property_application_type
                    }
                },
                None => PropertyApplicationType::Properties,
            }
        }
        None => PropertyApplicationType::Properties,
    }
}

fn get_entity_properties(
    forge_game_data: &crate::game_data::forge::GameData,
    entity: &Entity,
) -> Properties {
    let classname = entity.properties.get("classname");

    let forge_entity = forge_game_data
        .definitions
        .iter()
        .find(|forge_entity| Some(&forge_entity.class_name) == classname);

    let forge_entity = match forge_entity {
        Some(forge_entity) => forge_entity,
        None => {
            eprintln!("Failed to get forge entity for classname {:?}", classname);
            return Properties::default();
        }
    };

    let mut properties: HashMap<String, Property> = HashMap::new();

    let mut add_internal_property_integer = |property_name: &str| {
        if entity.properties.contains_key(property_name) {
            properties.insert(
                property_name.into(),
                crate::game_data::Property::Integer(
                    entity
                        .properties
                        .get(property_name)
                        .unwrap()
                        .parse::<i32>()
                        .unwrap(),
                ),
            );
        }
    };

    add_internal_property_integer("_tb_id");
    add_internal_property_integer("_tb_group");

    let mut add_internal_property_string = |property_name: &str| {
        if entity.properties.contains_key(property_name) {
            properties.insert(
                property_name.into(),
                crate::game_data::Property::String(
                    entity.properties.get(property_name).unwrap().clone(),
                ),
            );
        }
    };

    add_internal_property_string("_tb_type");
    add_internal_property_string("_tb_name");

    for forge_property in &forge_entity.properties {
        let key = &forge_property.name;
        let value = entity.properties.get(key);

        let value = match &forge_property.data {
            crate::game_data::forge::PropertyData::Integer(default) => {
                let value = value.map(|value| value.parse::<i32>());
                Property::Integer(value.unwrap_or(Ok(*default)).unwrap_or(*default))
            }
            crate::game_data::forge::PropertyData::Float(default) => {
                let value = value.map(|value| value.parse::<f32>());
                Property::Float(value.unwrap_or(Ok(*default)).unwrap_or(*default))
            }
            crate::game_data::forge::PropertyData::Vector3(default) => {
                let vector3 = match value {
                    Some(value) => match parse_vector3_property(value) {
                        Some(value) => value,
                        None => *default,
                    },
                    None => *default,
                };

                Property::Vector3(vector3)
            }
            crate::game_data::forge::PropertyData::String(default) => {
                let value = match value {
                    Some(value) => value,
                    None => default,
                };
                Property::String(value.clone())
            }
            crate::game_data::forge::PropertyData::Color(default) => {
                let color = match value {
                    Some(value) => match parse_color_property(value) {
                        Some(value) => value,
                        None => *default,
                    },
                    None => *default,
                };

                Property::Color(color)
            }
            crate::game_data::forge::PropertyData::Choices(_, default) => {
                let value = match value {
                    Some(value) => match value.parse::<i32>() {
                        Ok(value) => value,
                        Err(_) => *default,
                    },
                    None => *default,
                };

                Property::Choices(value)
            }
            crate::game_data::forge::PropertyData::Flags(_, default) => {
                let value = match value {
                    Some(value) => match value.parse::<i32>() {
                        Ok(value) => value,
                        Err(_) => *default,
                    },
                    None => *default,
                };

                Property::Choices(value)
            }
            crate::game_data::forge::PropertyData::TargetSource => Property::TargetSource,
            crate::game_data::forge::PropertyData::TargetDestination => Property::TargetDestination,
        };

        properties.insert(key.clone(), value);
    }

    Properties::new(properties)
}

fn parse_vector3_property(value: &str) -> Option<Vector3> {
    let mut comps = value.split(' ');

    let x: f32 = match parse_float(&mut comps) {
        Some(v) => v,
        None => return None,
    };

    let y: f32 = match parse_float(&mut comps) {
        Some(v) => v,
        None => return None,
    };

    let z: f32 = match parse_float(&mut comps) {
        Some(v) => v,
        None => return None,
    };

    Some(Vector3::new(x, y, z))
}

fn parse_color_property(value: &str) -> Option<Color> {
    let mut comps = value.split(' ');

    let r: f32 = match parse_float(&mut comps) {
        Some(v) => v,
        None => return None,
    };

    let g: f32 = match parse_float(&mut comps) {
        Some(v) => v,
        None => return None,
    };

    let b: f32 = match parse_float(&mut comps) {
        Some(v) => v,
        None => return None,
    };

    Some(Color::new(r, g, b))
}

fn parse_float(comps: &mut dyn std::iter::Iterator<Item = &str>) -> Option<f32> {
    match comps.next() {
        Some(v) => match v.parse::<f32>() {
            Ok(v) => Some(v),
            Err(_) => None,
        },
        None => None,
    }
}

fn get_entity_visual_geometry(
    quarchitect_game_data: &crate::game_data::GameData,
    texture_blacklist: &crate::types::TextureBlacklist,
    entity: &Entity,
    entity_geometry: &entity::Geometry,
) -> VisualGeometry {
    let classname = entity.properties.get("classname");

    let brush_data = match classname {
        Some(classname) => {
            let entity_definition = quarchitect_game_data
                .entities
                .iter()
                .find(|entity_definition| &entity_definition.classname == classname);

            match entity_definition {
                Some(entity_definition) => match &entity_definition.data {
                    crate::game_data::EntityData::Point(_) => return VisualGeometry::None,
                    crate::game_data::EntityData::Brush(_, brush_data) => brush_data,
                },
                None => {
                    println!("No entity definition for classname {:?}", classname);
                    return VisualGeometry::None;
                }
            }
        }
        None => panic!("No classname in entity"),
    };

    get_brush_entity_visual_geometry(entity, entity_geometry, brush_data, texture_blacklist)
}

fn get_brush_entity_visual_geometry(
    entity: &Entity,
    entity_geometry: &entity::Geometry,
    brush_data: &BrushData,
    texture_blacklist: &TextureBlacklist,
) -> VisualGeometry {
    println!("Visual type: {:?}", brush_data.visual_type);
    match brush_data.visual_type {
        crate::game_data::VisualType::Mesh => {
            // Collect brushes with this texture
            let textures: Vec<String> = entity
                .brushes
                .iter()
                .flat_map(|brush| brush.planes.iter().map(|plane| plane.texture.clone()))
                .collect();

            // Collect unique, non-blacklisted texture names
            let mut textures: Vec<Option<String>> = textures
                .clone()
                .into_iter()
                .enumerate()
                .filter(predicates::texture::unique_not_blacklisted(
                    &textures,
                    &texture_blacklist,
                ))
                .unzip::<usize, String, Vec<usize>, Vec<String>>()
                .1
                .into_iter()
                .map(Some)
                .collect();

            // Account for untextured brushes
            textures.push(None);

            // Build mesh surfaces for this texture
            let mesh_surfaces: Vec<MeshSurface> = textures
                .into_iter()
                .flat_map(build_brush_entity_texture_surface(
                    entity_geometry,
                    texture_blacklist,
                ))
                .collect();

            // If no mesh surfaces exist, there is no visual geometry
            if mesh_surfaces.is_empty() {
                return VisualGeometry::None;
            }

            // Return mesh-type visual geometry
            VisualGeometry::Mesh(VisualMesh::new(mesh_surfaces))
        }
        crate::game_data::VisualType::None => VisualGeometry::None,
    }
}

fn get_worldspawn_layer_visual_geometry(
    worldspawn_layer: &WorldspawnLayer,
    texture: &str,
    brush_geometry: &[crate::geo_builder::brush::Geometry],
) -> VisualGeometry {
    match worldspawn_layer.visual_type {
        VisualType::None => VisualGeometry::None,
        VisualType::Mesh => match build_brush_entity_texture_surface(
            &entity::Geometry::new(Vector3::default(), brush_geometry.to_vec()),
            &TextureBlacklist::default(),
        )(Some(texture.to_string()))
        {
            Some(mesh_surface) => VisualGeometry::Mesh(VisualMesh::new(vec![mesh_surface])),
            None => VisualGeometry::None,
        },
    }
}

fn get_worldspawn_layer_collision_geometry(
    worldspawn_layer: &WorldspawnLayer,
    brush_geometry: &[crate::geo_builder::brush::Geometry],
) -> CollisionGeometry {
    let layer_entity_geometry = entity::Geometry::new(Vector3::default(), brush_geometry.to_vec());

    match worldspawn_layer.collision_type {
        CollisionType::None => CollisionGeometry::None,
        CollisionType::Convex => get_entity_convex_collision(&layer_entity_geometry),
        CollisionType::Concave => get_entity_concave_collision(&layer_entity_geometry),
    }
}

fn build_brush_entity_texture_surface<'a>(
    entity_geometry: &'a entity::Geometry,
    texture_blacklist: &'a TextureBlacklist,
) -> impl Fn(Option<String>) -> Option<MeshSurface> + 'a {
    move |texture| {
        let (vertices, indices) = gather_entity_geometry(
            entity_geometry,
            Some(&predicates::brush::not_blacklisted(&texture_blacklist)),
            Some(&predicates::plane::has_texture(&texture)),
            Some(&predicates::vertex::unique),
        );

        if vertices.is_empty() {
            return None;
        }

        let verts: Vec<Vector3> = vertices.iter().map(|vertex| vertex.vertex).collect();
        let normals: Vec<Vector3> = vertices.iter().map(|vertex| vertex.normal).collect();
        let tangents: Vec<(Vector3, f32)> = vertices.iter().map(|vertex| vertex.tangent).collect();
        let uvs: Option<Vec<Vector2>> = vertices.iter().map(|vertex| vertex.uv).collect();
        let colors: Option<Vec<Color>> = vertices.iter().map(|vertex| vertex.color).collect();

        let mesh_surface =
            MeshSurface::new(texture, verts, normals, tangents, uvs, colors, indices);

        Some(mesh_surface)
    }
}

fn get_entity_collision_geometry(
    quarchitect_game_data: &crate::game_data::GameData,
    entity: &Entity,
    entity_geometry: &entity::Geometry,
) -> CollisionGeometry {
    let classname = entity.properties.get("classname");
    let brush_data = match classname {
        Some(classname) => {
            let entity_definition = quarchitect_game_data
                .entities
                .iter()
                .find(|entity_definition| &entity_definition.classname == classname);

            match entity_definition {
                Some(entity_definition) => match &entity_definition.data {
                    crate::game_data::EntityData::Point(_) => return CollisionGeometry::None,
                    crate::game_data::EntityData::Brush(_, brush_data) => brush_data,
                },
                None => {
                    println!("No entity definition for classname {:?}", classname);
                    return CollisionGeometry::None;
                }
            }
        }
        None => panic!("No classname in entity"),
    };

    match brush_data.collision_type {
        crate::game_data::CollisionType::Convex => get_entity_convex_collision(entity_geometry),
        crate::game_data::CollisionType::Concave => get_entity_concave_collision(entity_geometry),
        crate::game_data::CollisionType::None => CollisionGeometry::None,
    }
}

fn get_entity_convex_collision(entity_geometry: &entity::Geometry) -> CollisionGeometry {
    let convex_shapes: Vec<ConvexCollision> = entity_geometry
        .brush_geometry
        .iter()
        .map(|brush_geometry| {
            let points = brush_geometry
                .plane_geometry
                .iter()
                .flat_map(|brush_plane_geometry| {
                    brush_plane_geometry
                        .vertices
                        .iter()
                        .map(|vertex| vertex.vertex)
                })
                .collect::<Vec<Vector3>>();

            let points: Vec<Vector3> = points
                .iter()
                .enumerate()
                .flat_map(|(i, vertex)| {
                    if points
                        .iter()
                        .skip(i + 1)
                        .find(|comp| vertex == *comp)
                        .is_none()
                    {
                        Some(*vertex)
                    } else {
                        None
                    }
                })
                .collect();

            ConvexCollision::new(brush_geometry.center, points)
        })
        .collect();

    CollisionGeometry::convex(convex_shapes)
}

fn get_entity_concave_collision(entity_geometry: &entity::Geometry) -> CollisionGeometry {
    println!("Gathering concave collision geometry");
    let (vertices, indices) = gather_entity_geometry(
        entity_geometry,
        None,
        None,
        Some(&predicates::vertex::unique_position),
    );
    let collision_vertices: Vec<Vector3> = vertices.iter().map(|vertex| vertex.vertex).collect();

    if collision_vertices.is_empty() {
        return CollisionGeometry::None;
    }

    let concave_shapes = vec![ConcaveCollision::new(
        entity_geometry.center,
        collision_vertices,
        indices,
    )];

    CollisionGeometry::concave(concave_shapes)
}

fn gather_entity_geometry<'a>(
    entity_geometry: &'a entity::Geometry,
    brush_predicate: Option<&dyn Fn(&&crate::geo_builder::brush::Geometry) -> bool>,
    plane_predicate: Option<&dyn Fn(&&crate::geo_builder::brush_plane::Geometry) -> bool>,
    vertex_predicate: Option<&dyn Fn(usize, &Vertex, &[&Vertex]) -> bool>,
) -> (Vec<&'a Vertex>, Vec<usize>) {
    let brush_predicate = match brush_predicate {
        Some(brush_predicate) => brush_predicate,
        None => &|_: &&crate::geo_builder::brush::Geometry| true,
    };

    let brush_geometry: Vec<(Vec<&Vertex>, Vec<usize>)> = entity_geometry
        .brush_geometry
        .iter()
        .filter(&brush_predicate)
        .map(|brush_geometry| {
            gather_brush_geometry(brush_geometry, plane_predicate, vertex_predicate)
        })
        .collect();

    let vertices: Vec<&Vertex> = brush_geometry
        .iter()
        .flat_map(|(vertices, _indices)| (*vertices).clone())
        .collect();

    let mut index_offset: usize = 0;
    let indices: Vec<usize> = brush_geometry
        .iter()
        .flat_map(|(vertices, indices)| {
            let indices = indices
                .clone()
                .into_iter()
                .map(move |index| index + index_offset);

            index_offset += vertices.len();

            indices
        })
        .collect();

    (vertices, indices)
}

fn gather_brush_geometry<'a>(
    brush_geometry: &'a brush::Geometry,
    plane_predicate: Option<&dyn Fn(&&crate::geo_builder::brush_plane::Geometry) -> bool>,
    vertex_predicate: Option<&dyn Fn(usize, &Vertex, &[&Vertex]) -> bool>,
) -> (Vec<&'a Vertex>, Vec<usize>) {
    let plane_geometry = &brush_geometry.plane_geometry;

    let plane_predicate = match plane_predicate {
        Some(plane_predicate) => plane_predicate,
        None => &|_: &&crate::geo_builder::brush_plane::Geometry| true,
    };

    let vertices: Vec<&Vertex> = plane_geometry
        .iter()
        .filter(plane_predicate)
        .flat_map(move |plane_geometry| &plane_geometry.vertices)
        .collect();

    let mut index_offset: usize = 0;

    let concat_indices = |plane_geometry: &crate::geo_builder::brush_plane::Geometry| {
        let indices = plane_geometry
            .indices
            .clone()
            .into_iter()
            .map(move |index| index + index_offset);

        index_offset += plane_geometry.vertices.len();

        indices
    };

    let indices: Vec<usize> = plane_geometry
        .iter()
        .filter(plane_predicate)
        .flat_map(concat_indices)
        .collect();

    match vertex_predicate {
        Some(vertex_predicate) => filter_vertices(&vertices, indices, vertex_predicate),
        None => (vertices, indices),
    }
}

fn filter_vertices<'a>(
    vertices: &[&'a Vertex],
    indices: Vec<usize>,
    predicate: &dyn Fn(usize, &Vertex, &[&Vertex]) -> bool,
) -> (Vec<&'a Vertex>, Vec<usize>) {
    let mut indices = indices;
    let mut new_indices: Vec<usize> = Vec::new();
    let mut new_vertices: Vec<&Vertex> = Vec::new();

    for (i, vertex) in vertices.iter().enumerate() {
        if predicate(i, vertex, &vertices) {
            new_indices.push(i);
            new_vertices.push(vertex);
        } else {
            let position = vertices
                .iter()
                .position(|comp| comp.vertex == vertex.vertex)
                .unwrap();

            indices = indices
                .iter()
                .map(|index| if *index == i { position } else { *index })
                .collect();
        }
    }

    let indices: Vec<usize> = indices
        .iter()
        .flat_map(|index| new_indices.iter().position(|comp| comp == index))
        .collect();

    (new_vertices, indices)
}
