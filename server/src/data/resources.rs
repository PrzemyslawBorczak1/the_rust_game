use bevy::prelude::*;

#[derive(States, Debug, Hash, Eq, PartialEq, Clone, Default)]
pub enum GameState {
    #[default]
    Menu,
    NewGame,
    LoadGame,
    Game,
    History,
}

#[derive(Debug, Default, Resource)]
pub struct FetchGamePath {
    pub id_texture: String,
    pub vec_provinces: String,
    pub vec_country: String,
}

#[derive(Debug, Default, Resource)]
pub struct SaveGamePath {
    pub id_texture: String,
    pub vec_provinces: String,
    pub vec_country: String,
}
