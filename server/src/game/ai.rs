use bevy::{ecs::world, prelude::*};
use shared::resources::GameWorld;

use crate::game::net::types::ActiveClients;

#[derive(States, Debug, Hash, Eq, PartialEq, Clone, Default)]
pub enum AiState {
    #[default]
    Disabled,
    Runing,
}

#[derive(Resource, Default)]
pub struct AiCountries(pub Vec<u32>);

#[derive(Resource, Default)]
pub struct AiTimer(pub Timer, pub bool);

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AiCountries>()
            .insert_resource(AiTimer(
                Timer::from_seconds(2.0, TimerMode::Repeating),
                false,
            ))
            .init_state::<AiState>()
            .add_systems(OnEnter(AiState::Runing), start_ai)
            .add_systems(Update, make_ai_move.run_if(in_state(AiState::Runing)));
    }
}

fn start_ai(
    mut world: ResMut<GameWorld>,
    mut ai_countries: ResMut<AiCountries>,
    mut timer: ResMut<AiTimer>,
) {
    let mut i = 0u32;

    for c in &mut world.countries {
        if !c.is_taken {
            ai_countries.0.push(i);
        }
        c.is_taken = true;
        i += 1;
    }

    timer.1 = true;
}

pub fn make_ai_move(world: Res<GameWorld>) {}
