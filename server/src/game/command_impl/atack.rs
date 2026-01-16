use std::net::SocketAddr;

use bevy::prelude::*;

use shared::{
    commands_client::{CommandClient, basic::*},
    commands_server::basic::*,
    resources::{GameWorld, NO_OWNER, TERRAIN_WATER},
};

use super::helpers::*;

use crate::game::net::types::{ActiveClients, OutCmd, Target};

pub trait ExecuteAttack {
    fn execute(
        &self,
        world: &mut GameWorld,
        id_country: Option<u32>,
        addr: SocketAddr,
    ) -> Option<Vec<OutCmd>>;
}

fn validate_attack_basic(
    attack: &Attack,
    world: &GameWorld,
    id: u32,
    addr: SocketAddr,
) -> Option<Vec<OutCmd>> {
    let from = attack.from_province as usize;
    let to_u32 = attack.to_province;

    if world.provinces[from].owner_id != id {
        return log_only(
            addr,
            "You cant only move army from you own province".to_string(),
        );
    }
    if !world.id.adjacency[from].contains(&to_u32) {
        return log_only(
            addr,
            "You cant only move army to adjacenc provinces".to_string(),
        );
    }
    if world.provinces[from].army == 0 {
        return log_only(addr, "You dont have army here".to_string());
    }
    None
}

fn validate_attack_war(
    attack: &Attack,
    world: &GameWorld,
    id: u32,
    addr: SocketAddr,
) -> Option<Vec<OutCmd>> {
    let enemy = world.provinces[attack.to_province as usize].owner_id;
    if enemy != NO_OWNER && enemy != id && !world.countries[id as usize].war.contains(&enemy) {
        return log_only(
            addr,
            "You have to be at war with another country to send there army".to_string(),
        );
    }
    None
}

fn take_army(world: &mut GameWorld, from: u32) -> u32 {
    let i = from as usize;
    let army = world.provinces[i].army;
    world.provinces[i].army = 0;
    army
}

fn resolve_battle(world: &mut GameWorld, to: u32, id: u32, army: u32) -> &'static str {
    let i = to as usize;
    let enemy_army = world.provinces[i].army;

    if enemy_army > army {
        world.provinces[i].army = enemy_army - army;
        "All of your solders died"
    } else {
        world.provinces[i].army = army - enemy_army;
        world.provinces[i].owner_id = id;
        "You have counquerd another province :)"
    }
}

fn execute_attack_with_country(
    attack: &Attack,
    world: &mut GameWorld,
    id: u32,
    addr: SocketAddr,
) -> Option<Vec<OutCmd>> {
    if let Some(err) = validate_attack_basic(attack, world, id, addr) {
        return Some(err);
    }
    if let Some(err) = validate_attack_war(attack, world, id, addr) {
        return Some(err);
    }

    let mut out = Vec::new();
    let from = attack.from_province;
    let to = attack.to_province;

    let army = take_army(world, from);
    push_update_province_all(&mut out, world, from)?;

    if world.provinces[to as usize].terrain_type == TERRAIN_WATER {
        return append_log(addr, "All of your army drowned".to_string(), out);
    }

    let enemy = world.provinces[to as usize].owner_id;
    if enemy == id {
        world.provinces[to as usize].army = army;
        push_update_province_all(&mut out, world, to)?;
        return append_log(addr, "You have moved your army".to_string(), out);
    }

    if enemy == NO_OWNER {
        world.provinces[to as usize].army = army;
        world.provinces[to as usize].owner_id = id;
        push_update_province_all(&mut out, world, to)?;
        return append_log(
            addr,
            "You have counquerd another province :)".to_string(),
            out,
        );
    }

    let msg = resolve_battle(world, to, id, army);
    push_update_province_all(&mut out, world, to)?;
    append_log(addr, msg.to_string(), out)
}

impl ExecuteAttack for Attack {
    fn execute(
        &self,
        world: &mut GameWorld,
        id_country: Option<u32>,
        addr: SocketAddr,
    ) -> Option<Vec<OutCmd>> {
        let id = match id_country {
            Some(x) => x,
            None => return log_only(addr, "Choose country first".to_string()),
        };
        execute_attack_with_country(self, world, id, addr)
    }
}
