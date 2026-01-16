use bevy::prelude::*;
use shared::{
    commands_client::CommandClient,
    resources::{GameWorld, NO_OWNER},
};

use crate::{
    data::GameState,
    game::net::types::{NetOutbox, OutCmd, Target},
};
#[derive(Resource)]
pub struct GlobalTimer(pub Timer, pub bool);

pub struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, add_gold.run_if(in_state(GameState::Game)))
            .insert_resource(GlobalTimer(
                Timer::from_seconds(2.0, TimerMode::Repeating),
                false,
            ));
    }
}

fn add_gold(
    mut world: ResMut<GameWorld>,
    mut t: ResMut<GlobalTimer>,
    time: Res<Time>,
    out: ResMut<NetOutbox>,
) {
    if !t.1 {
        return;
    }

    t.0.tick(time.delta());

    if !t.0.just_finished() {
        return;
    }

    let w: &mut GameWorld = world.as_mut();

    let provinces = &w.provinces;
    let countries = &mut w.countries;

    for pr in provinces.iter() {
        let owner = pr.owner_id;
        if owner == NO_OWNER {
            continue;
        }

        countries[owner as usize].gold += pr.gold_production;
    }
    let msg = match CommandClient::UpdateCountries(world.countries.clone()).serialize() {
        Ok(x) => x,
        Err(e) => {
            error!("Couldnt serialize UpdateCountry");
            return;
        }
    };

    if let Err(_) = out.0.send(OutCmd::Send {
        targets: Target::All,
        msg: msg,
    }) {
        error!("Couldnt send UpdateCountry");
    }
}
