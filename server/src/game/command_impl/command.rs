use std::{net::SocketAddr, process::Command};

use shared::{commands_server::CommandServer, resources::GameWorld};

use super::basic::*;
use crate::game::{command_impl::atack::ExecuteAttack, net::types::{ActiveClients, OutCmd}};

pub trait Execute {
    fn execute(
        &self,
        world: &mut GameWorld,
        active: &mut ActiveClients,
        addr: SocketAddr,
    ) -> Option<Vec<OutCmd>>;
}

impl Execute for CommandServer {
    fn execute(
        &self,
        world: &mut GameWorld,
        active: &mut ActiveClients,
        addr: SocketAddr,
    ) -> Option<Vec<OutCmd>> {
        match self {
            CommandServer::Attack(a) => {
                let idx = match active.0.get(&addr) {
                    None => {
                        return None;
                    }
                    Some(x) => x,
                };
                a.execute(world, *idx, addr)
            }
            CommandServer::ChooseCountry(cc) => cc.execute(world, active, addr),
            CommandServer::BuyBank(bb) => bb.execute(world, active, addr),
            CommandServer::BuyArmy(ba) => ba.execute(world, active, addr),
            CommandServer::MakePeace(mp) => mp.execute(world, active, addr),
            CommandServer::StartWar(sw) => sw.execute(world, active, addr),
        }
    }
}
