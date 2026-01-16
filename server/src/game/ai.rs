use bevy::{ecs::world, prelude::*};
use rand::{Rng, random};
use shared::{
    commands_client::{CommandClient, basic::UpdateProvince},
    resources::{GameWorld, TERRAIN_WATER},
};

use crate::game::{
    command_impl::helpers::make_send,
    net::types::{ActiveClients, NetOutbox, OutCmd, Target},
};

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

fn make_ai_move(
    mut world: ResMut<GameWorld>,
    sender: Res<NetOutbox>,
    ai_country: Res<AiCountries>,
    mut timer: ResMut<AiTimer>,
    time: Res<Time>,
) {
    if !timer.0.tick(time.delta()).just_finished() || timer.1 == false {
        return;
    }

    for country in &ai_country.0 {
        make_move(&mut world, &sender, country.clone());
    }
}

fn make_move(world: &mut GameWorld, sender: &NetOutbox, country: u32) {
    if let Some(src) = find_owned_province_idx(world, country) {
        let neighbours = &world.id.adjacency[src];
        if neighbours.is_empty() {
            return;
        }

        let dst_id: u32 = neighbours[rand::rng().random_range(0..neighbours.len())];
        let dst = dst_id as usize;

        world.provinces[src].army = 1;

        if world.provinces[dst].terrain_type != TERRAIN_WATER {
            if world.provinces[dst].army == 0 {
                world.provinces[dst].owner_id = country;
                world.provinces[dst].army = 1;
            } else {
                world.provinces[dst].army -= 1;
            }
        }

        let msg1 = make_send(
            Target::All,
            CommandClient::UpdateProvince(UpdateProvince {
                id: dst_id,
                province: world.provinces[dst].clone(),
            }),
        );

        let msg2 = make_send(
            Target::All,
            CommandClient::UpdateProvince(UpdateProvince {
                id: src as u32,
                province: world.provinces[src].clone(),
            }),
        );

        if let Some(m) = msg1 {
            let _ = sender.0.send(m);
        }
        if let Some(m) = msg2 {
            let _ = sender.0.send(m);
        }
    }
}

fn find_owned_province_idx(world: &GameWorld, country: u32) -> Option<usize> {
    let n = world.provinces.len();
    if n == 0 {
        return None;
    }

    let start = (rand::random::<u32>() as usize) % n;

    for k in 0..n {
        let i = (start + k) % n;
        if world.provinces[i].owner_id == country {
            return Some(i);
        }
    }
    None
}
