use crate::GameState;
use crate::menu::{GameStartType, NewGameData};
use bevy::prelude::*;

pub struct SetMapPlugin;

impl Plugin for SetMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), set_map);
    }
}

fn set_map(game_start_type: Res<GameStartType>) {
    match &*game_start_type {
        GameStartType::NewGame(data) => new_game_setup(&data),
        GameStartType::Load(_) => load_game_setup(),
        // todo error
        GameStartType::Undefined => {}
    }
}


fn new_game_setup(data: &NewGameData) {
    println!("new game");
    println!("{:?}", data.id_path);
    println!("{:?}", data.texture_path);
}

fn load_game_setup() {
    println!("load game");
}
