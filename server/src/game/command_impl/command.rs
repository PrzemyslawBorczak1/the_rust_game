use shared::{ commands_server::CommandServer, resources::GameWorld};

use crate::game::net::types::OutCmd;

pub trait Execute {
    fn execute(&self, world: &mut GameWorld) ->Option<Vec<OutCmd>>;
}

impl Execute for CommandServer {
    fn execute(&self, world: &mut GameWorld) -> Option<Vec<OutCmd>> {
        match self {
            CommandServer::UpgradeProvince(s) => s.execute(world),
            CommandServer::BuyArmy(s) => s.execute(world),
        }
    }
}
