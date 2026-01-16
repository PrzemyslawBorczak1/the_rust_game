use std::fs;

use bevy::prelude::*;
use shared::{commands_server::basic::Attack, resources::GameWorld};

const path: &str = "assets\\history\\history.json";

#[derive(Default, Resource)]
pub struct History {
    change_id_owner: Vec<(u32, u32)>,
    watched_province_id: u32,
    last_owner: u32,
    is_watching: bool,
}

pub struct HistoryPlugin;

impl Plugin for HistoryPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<History>();
    }
}

impl History {
    pub fn set_watch(&mut self, atack: &Attack, world: &GameWorld) {
        self.is_watching = true;
        self.watched_province_id = atack.to_province;
        self.last_owner = world.provinces[atack.to_province as usize].owner_id;
    }

    pub fn consolidate(&mut self, world: &GameWorld) {
        if self.is_watching == false {
            return;
        }

        self.is_watching = false;
        let new_owner = world.provinces[self.watched_province_id as usize].owner_id;
        if new_owner != self.last_owner {
            info!("Zmaina");

            self.change_id_owner
                .push((self.watched_province_id, new_owner));
        }
    }

    pub fn save(&self) {
        let history = match serde_json::to_string(&self.change_id_owner) {
            Ok(h) => h,
            Err(e) => {
                error!("Couldnt serialize history: {e}");
                return;
            }
        };

        if let Err(e) = fs::write(path, history) {
            error!("Couldn save file to path [{path}] error: {}", e);
        }
    }
}
