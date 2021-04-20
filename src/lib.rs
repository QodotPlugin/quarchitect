pub mod game_data;
pub mod scene_tree;
pub mod wad;

mod geo_builder;
mod layer_filter;
mod map;
mod types;

pub use types::{
    Color, Mat2, Quat, Texture, TextureBlacklist, TextureInfo, Vector2, Vector3, Vertex,
};

use std::error::Error;
use std::{fmt, fs};

#[derive(Debug)]
pub struct Config {
    map_file: String,
    texture_info: TextureInfo,
    texture_blacklist: TextureBlacklist,
    forge_game_data: game_data::forge::GameData,
    quarchitect_game_data: game_data::GameData,
}

impl Config {
    pub fn new(
        map_file: &str,
        texture_info: TextureInfo,
        texture_blacklist: TextureBlacklist,
        forge_game_data: game_data::forge::GameData,
        quarchitect_game_data: game_data::GameData,
    ) -> Config {
        let map_file = map_file.into();
        Config {
            map_file,
            texture_info,
            texture_blacklist,
            forge_game_data,
            quarchitect_game_data,
        }
    }
}

#[derive(Debug)]
pub struct QuarchitectError(&'static str);

impl fmt::Display for QuarchitectError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})", self.0)
    }
}
impl Error for QuarchitectError {}

pub fn run(config: Config) -> Result<Vec<scene_tree::SceneTreeNode>, Box<dyn Error>> {
    println!("TODO-3: Profile performance cost centers against ad_sepulcher.map, multithread with rayon");

    // Parse map into tokens and entities
    println!("Parse map");
    let file_string = fs::read_to_string(config.map_file)?;
    let tokens = map::quake::tokenizer::run(file_string);
    let (_token_paths, entities) = map::quake::parser::run(&tokens)?;

    // Build geometry
    let entity_geometry = geo_builder::run(&config.texture_info, &entities);

    // Couple entities to their geometry
    let entity_data: Vec<(map::quake::Entity, geo_builder::entity::Geometry)> = entities
        .into_iter()
        .zip(entity_geometry.into_iter())
        .collect();

    // Split layers out of worldspawn
    let (entity_data, worldspawn_layer_data) = layer_filter::run(entity_data);

    // Build engine representation
    let scene_tree = scene_tree::run(
        &config.forge_game_data,
        &config.quarchitect_game_data,
        &config.texture_blacklist,
        &entity_data,
        &worldspawn_layer_data,
    );

    Ok(scene_tree)
}

pub fn run_diff(file_a: &str, file_b: &str) -> Result<(), Box<dyn Error>> {
    let file_a_string = fs::read_to_string(file_a)?;
    let file_b_string = fs::read_to_string(file_b)?;

    let file_a_tokens = map::quake::tokenizer::run(file_a_string);
    let file_b_tokens = map::quake::tokenizer::run(file_b_string);

    let diff = map::quake::tokenizer::diff_tokens(&file_a_tokens, &file_b_tokens);
    println!("{:#?}", diff);

    Ok(())
}
