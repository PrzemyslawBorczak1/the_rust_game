use bevy::prelude::*;

use crate::{data::GameState, history};

pub struct HistoryGraphics;

impl Plugin for HistoryGraphics {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::History), startup);
    }
}

fn startup(commands: Commands) {
    
}
