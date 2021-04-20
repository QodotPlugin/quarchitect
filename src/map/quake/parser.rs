use std::collections::HashMap;

use super::Brush;
use super::BrushPlane;
use super::Entity;
use super::Token;
use crate::QuarchitectError;

#[derive(Debug)]
pub struct EntityPath {
    entity_idx: usize,
}

impl EntityPath {
    pub fn new(entity_idx: usize) -> EntityPath {
        EntityPath { entity_idx }
    }
}

#[derive(Debug)]
pub struct PropertyPath {
    entity_idx: usize,
    property_name: String,
}

impl PropertyPath {
    pub fn new(entity_idx: usize, property_name: String) -> PropertyPath {
        PropertyPath {
            entity_idx,
            property_name,
        }
    }
}

#[derive(Debug)]
pub struct BrushPath {
    entity_idx: usize,
    brush_idx: usize,
}

impl BrushPath {
    pub fn new(entity_idx: usize, brush_idx: usize) -> BrushPath {
        BrushPath {
            entity_idx,
            brush_idx,
        }
    }
}

#[derive(Debug)]
pub struct BrushPlanePath {
    entity_idx: usize,
    brush_idx: usize,
    plane_idx: usize,
}

impl BrushPlanePath {
    pub fn new(entity_idx: usize, brush_idx: usize, plane_idx: usize) -> BrushPlanePath {
        BrushPlanePath {
            entity_idx,
            brush_idx,
            plane_idx,
        }
    }
}

#[derive(Debug)]
enum ParseScope {
    File,
    Entity(EntityPath),
    Brush(BrushPath),
}

impl ParseScope {
    fn file() -> ParseScope {
        ParseScope::File
    }

    fn entity(entity_idx: usize) -> ParseScope {
        ParseScope::Entity(EntityPath::new(entity_idx))
    }

    fn brush(entity_idx: usize, brush_idx: usize) -> ParseScope {
        ParseScope::Brush(BrushPath::new(entity_idx, brush_idx))
    }
}

#[derive(Debug)]
pub enum TokenPath {
    Entity(EntityPath),
    Property(PropertyPath),
    Brush(BrushPath),
    BrushPlane(BrushPlanePath),
}

impl TokenPath {
    fn entity(entity_idx: usize) -> TokenPath {
        TokenPath::Entity(EntityPath::new(entity_idx))
    }

    fn property(entity_idx: usize, property_name: String) -> TokenPath {
        TokenPath::Property(PropertyPath::new(entity_idx, property_name))
    }

    fn brush(entity_idx: usize, brush_idx: usize) -> TokenPath {
        TokenPath::Brush(BrushPath::new(entity_idx, brush_idx))
    }

    fn brush_plane(entity_idx: usize, brush_idx: usize, brush_plane_idx: usize) -> TokenPath {
        TokenPath::BrushPlane(BrushPlanePath::new(entity_idx, brush_idx, brush_plane_idx))
    }
}

pub fn run(
    tokens: &[Token],
) -> Result<(HashMap<&Token, TokenPath>, Vec<Entity>), QuarchitectError> {
    println!("TODO-3: Rewrite with nom");

    let mut scope = ParseScope::file();

    let mut token_paths: HashMap<&Token, TokenPath> = HashMap::new();
    let mut entities: Vec<Entity> = Vec::new();

    for token in tokens.iter() {
        match token {
            Token::OpenBrace => match scope {
                ParseScope::File => {
                    let entity_id = entities.len();
                    scope = ParseScope::entity(entity_id);
                    entities.push(Entity::new());
                    token_paths.insert(token, TokenPath::entity(entity_id));
                }
                ParseScope::Entity(entity_path) => {
                    let brushes = &mut entities.last_mut().unwrap().brushes;
                    let brush_idx = brushes.len();
                    scope = ParseScope::brush(entity_path.entity_idx, brush_idx);
                    brushes.push(Brush::new());
                    token_paths.insert(token, TokenPath::brush(entity_path.entity_idx, brush_idx));
                }
                ParseScope::Brush(_) => return Err(QuarchitectError("Open brace in brush scope")),
            },
            Token::CloseBrace => match scope {
                ParseScope::File => return Err(QuarchitectError("Close brace in file scope")),
                ParseScope::Entity(_entity_id) => scope = ParseScope::file(),
                ParseScope::Brush(brush_path) => scope = ParseScope::entity(brush_path.entity_idx),
            },
            Token::Property(p) => match &scope {
                ParseScope::File => return Err(QuarchitectError("Property in file scope")),
                ParseScope::Entity(entity_path) => {
                    let k = &p.key;
                    let v = &p.value;
                    entities
                        .last_mut()
                        .unwrap()
                        .properties
                        .insert(k.clone(), v.clone());
                    token_paths.insert(
                        token,
                        TokenPath::property(entity_path.entity_idx, k.clone()),
                    );
                }
                ParseScope::Brush(_) => return Err(QuarchitectError("Property in brush scope")),
            },
            Token::BrushPlane(bp) => match &scope {
                ParseScope::File => return Err(QuarchitectError("Brushplane in file scope")),
                ParseScope::Entity(_) => {
                    return Err(QuarchitectError("Brushplane in entity scope"))
                }
                ParseScope::Brush(brush_path) => {
                    let brush_planes = &mut entities
                        .last_mut()
                        .unwrap()
                        .brushes
                        .last_mut()
                        .unwrap()
                        .planes;
                    let brush_plane_id = brush_planes.len();
                    let brush_plane = BrushPlane::new(bp.as_str())?;
                    brush_planes.push(brush_plane);
                    token_paths.insert(
                        token,
                        TokenPath::brush_plane(
                            brush_path.entity_idx,
                            brush_path.brush_idx,
                            brush_plane_id,
                        ),
                    );
                }
            },
            _ => (),
        }
    }

    Ok((token_paths, entities))
}
