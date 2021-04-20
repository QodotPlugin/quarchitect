#[derive(Debug)]
pub struct GameData {
    pub entities: Vec<crate::game_data::Entity>,
    pub worldspawn_layers: Vec<crate::game_data::WorldspawnLayer>,
}
