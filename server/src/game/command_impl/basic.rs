use bevy::prelude::*;

use shared::{
    commands_client::CommandClient, commands_client::basic::ChangeProvince,
    commands_server::basic::*, resources::GameWorld,
};

use crate::game::net::types::{OutCmd, Target};

use super::command::Execute;

impl Execute for BuyArmy {
    fn execute(&self, _world: &mut GameWorld) -> Option<Vec<OutCmd>> {
        let command = match CommandClient::ChangeProvince(vec![ChangeProvince {}]).serialize() {
            Ok(c) => c,
            Err(e) => {
                error!("Couldnt serialize {e}");
                return None;
            }
        };

        Some(vec![OutCmd::Send {
            targets: Target::All,
            msg: command,
        }])
    }
}

impl Execute for UpgradeProvince {
    fn execute(&self, _world: &mut GameWorld) -> Option<Vec<OutCmd>> {
        let command = match CommandClient::ChangeProvince(vec![ChangeProvince {}]).serialize() {
            Ok(c) => c,
            Err(e) => {
                error!("Couldnt serialize {e}");
                return None;
            }
        };

        Some(vec![OutCmd::Send {
            targets: Target::All,
            msg: command,
        }])
    }
}
