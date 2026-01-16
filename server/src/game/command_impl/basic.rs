use std::net::SocketAddr;

use bevy::{ecs::world, prelude::*};

use shared::{
    commands_client::{CommandClient, basic::*},
    commands_server::basic::*,
    resources::{GameWorld, NO_OWNER, TERRAIN_WATER},
};

use crate::game::net::types::{ActiveClients, OutCmd, Target};

use super::helpers::*;

pub trait ExecuteChooseCountry {
    fn execute(
        &self,
        world: &mut GameWorld,
        active: &mut ActiveClients,
        addr: SocketAddr,
    ) -> Option<Vec<OutCmd>>;
}

fn choose_country(
    active: &mut ActiveClients,
    addr: SocketAddr,
    id: u32,
    world: &mut GameWorld,
) -> Option<Vec<OutCmd>> {
    if id == NO_OWNER {
        return log_only(addr, "Selected province doesnt have owner".to_string());
    }
    if active.0.values().any(|v| v == &Some(id)) {
        return log_only(addr, "Country already taken".to_string());
    }
    if world.countries[id as usize].is_taken {
        return log_only(addr, "Country already taken".to_string());
    }

    active.0.insert(addr, Some(id));
    log_only(addr, "Country chosen succesfully".to_string())
}

impl ExecuteChooseCountry for ChooseCountry {
    fn execute(
        &self,
        world: &mut GameWorld,
        active: &mut ActiveClients,
        addr: SocketAddr,
    ) -> Option<Vec<OutCmd>> {
        choose_country(active, addr, self.0, world)
    }
}

pub trait ExecuteBuyBank {
    fn execute(
        &self,
        world: &mut GameWorld,
        active: &mut ActiveClients,
        addr: SocketAddr,
    ) -> Option<Vec<OutCmd>>;
}

fn bank_cost_for_level(level: u32) -> Option<u32> {
    let prices = [100u32, 10000u32, 50000u32];
    let idx = level.saturating_sub(1) as usize;
    prices.get(idx).copied()
}

fn buy_bank(
    world: &mut GameWorld,
    province_id: u32,
    owner: u32,
    addr: SocketAddr,
) -> Option<Vec<OutCmd>> {
    if province_id == NO_OWNER {
        return log_only(addr, "Choose country first".to_string());
    }

    let province = world.provinces[province_id as usize].clone();
    if owner != province.owner_id {
        return log_only(addr, "Better buy bank in your country".to_string());
    }
    if province.level > 2 {
        return log_only(addr, "You have max level bank".to_string());
    }

    let needs = match bank_cost_for_level(province.level) {
        Some(x) => x,
        None => return log_only(addr, "Invalid bank level".to_string()),
    };
    if world.countries[owner as usize].gold < needs {
        return log_only(addr, "You dont have enough gold".to_string());
    }

    world.countries[owner as usize].gold -= needs;
    world.provinces[province_id as usize].level += 1;
    world.provinces[province_id as usize].gold_production *= 2;

    let mut out = Vec::new();
    push_cmd_all(
        &mut out,
        CommandClient::UpdateCountries(world.countries.clone()),
    )?;
    push_cmd_all(
        &mut out,
        CommandClient::UpdateProvince(UpdateProvince {
            id: province_id,
            province: world.provinces[province_id as usize].clone(),
        }),
    )?;
    append_log(addr, "You succesffully bought bank".to_string(), out)
}

impl ExecuteBuyBank for BuyBank {
    fn execute(
        &self,
        world: &mut GameWorld,
        active: &mut ActiveClients,
        addr: SocketAddr,
    ) -> Option<Vec<OutCmd>> {
        let owner = match require_selected_country(active, addr) {
            Ok(x) => x,
            Err(out) => return Some(out),
        };
        buy_bank(world, self.0, owner, addr)
    }
}

pub trait ExecuteBuyArmy {
    fn execute(
        &self,
        world: &mut GameWorld,
        active: &mut ActiveClients,
        addr: SocketAddr,
    ) -> Option<Vec<OutCmd>>;
}

fn buy_army(
    world: &mut GameWorld,
    province_id: u32,
    owner: u32,
    addr: SocketAddr,
) -> Option<Vec<OutCmd>> {
    if province_id == NO_OWNER {
        return log_only(addr, "Choose country first".to_string());
    }

    let province = world.provinces[province_id as usize].clone();
    if owner != province.owner_id {
        return log_only(addr, "Better buy army in your country".to_string());
    }

    let price = 10u32;
    if world.countries[owner as usize].gold < price {
        return log_only(addr, "You dont have enough gold".to_string());
    }

    let army_amount = world.countries[owner as usize].gold / price;
    world.countries[owner as usize].gold -= army_amount * price;
    world.provinces[province_id as usize].army += army_amount;

    let mut out = Vec::new();
    push_cmd_all(
        &mut out,
        CommandClient::UpdateCountries(world.countries.clone()),
    )?;
    push_cmd_all(
        &mut out,
        CommandClient::UpdateProvince(UpdateProvince {
            id: province_id,
            province: world.provinces[province_id as usize].clone(),
        }),
    )?;
    append_log(addr, "You succesffully bought army".to_string(), out)
}

impl ExecuteBuyArmy for BuyArmy {
    fn execute(
        &self,
        world: &mut GameWorld,
        active: &mut ActiveClients,
        addr: SocketAddr,
    ) -> Option<Vec<OutCmd>> {
        let owner = match require_selected_country(active, addr) {
            Ok(x) => x,
            Err(out) => return Some(out),
        };
        buy_army(world, self.0, owner, addr)
    }
}

pub trait ExecuteMakePeace {
    fn execute(
        &self,
        world: &mut GameWorld,
        active: &mut ActiveClients,
        addr: SocketAddr,
    ) -> Option<Vec<OutCmd>>;
}

impl ExecuteMakePeace for MakePeace {
    fn execute(
        &self,
        world: &mut GameWorld,
        active: &mut ActiveClients,
        addr: SocketAddr,
    ) -> Option<Vec<OutCmd>> {
        let id = match require_selected_country(active, addr) {
            Ok(x) => x,
            Err(e) => return Some(e),
        };

        if self.0 == NO_OWNER {
            return log_only(addr, "Make peace with real country".to_string());
        }

        world.countries[id as usize].war.remove(&self.0);

        let mut ret = vec![];

        push_cmd_all(
            &mut ret,
            CommandClient::UpdateCountries(world.countries.clone()),
        );

        return append_log(addr, "You succesfully made peace".to_string(), ret);
    }
}

pub trait ExecuteStartWar {
    fn execute(
        &self,
        world: &mut GameWorld,
        active: &mut ActiveClients,
        addr: SocketAddr,
    ) -> Option<Vec<OutCmd>>;
}

impl ExecuteStartWar for StartWar {
    fn execute(
        &self,
        world: &mut GameWorld,
        active: &mut ActiveClients,
        addr: SocketAddr,
    ) -> Option<Vec<OutCmd>> {
        let id = match require_selected_country(active, addr) {
            Ok(x) => x,
            Err(e) => return Some(e),
        };

        if self.0 == NO_OWNER {
            return log_only(addr, "Start war with real country".to_string());
        }

        world.countries[id as usize].war.insert(self.0);

        let mut ret = vec![];

        push_cmd_all(
            &mut ret,
            CommandClient::UpdateCountries(world.countries.clone()),
        );

        return append_log(addr, "You succesfully started war".to_string(), ret);
    }
}
