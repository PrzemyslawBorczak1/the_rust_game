use super::{helper_country::generate_country, helper_map::generate_province_map_png};
use bevy::prelude::*;

use crate::{
    data::{FetchGamePath, GameState},
    startup_game::{graphics::LoadingGraphics, new_game::helper_province::generate_province},
};

pub struct NewGamePlugin;

impl Plugin for NewGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::NewGame), startup)
            .add_systems(OnEnter(GameState::NewGame), create_map_id);
    }
}

fn startup(mut graphics: ResMut<NextState<LoadingGraphics>>) {
    graphics.set(LoadingGraphics::Show);
}

fn create_map_id(fetch: Res<FetchGamePath>, mut game: ResMut<NextState<GameState>>) {
    if let Ok((map,adj)) = generate_province_map_png(1000, 1000, 100, 42, &fetch.id_texture) {
        if let Err(_) = generate_province(100, 42, adj, &fetch.vec_provinces, 4) {
            error!("Creating provinces went wrong.");
        }
        if let Err(_) = generate_country(&fetch.vec_country) {
            error!("Creating countries went wrong.");
        }
    } else {
        error!("Creating map went wrong.");
    }

    game.set(GameState::LoadGame);
}
