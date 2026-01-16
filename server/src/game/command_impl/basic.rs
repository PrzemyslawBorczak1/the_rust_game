use std::net::SocketAddr;

use bevy::prelude::*;

use shared::{
    commands_client::{CommandClient, basic::*},
    commands_server::basic::*,
    resources::{GameWorld, NO_OWNER, TERRAIN_WATER},
};

use crate::game::net::types::{ActiveClients, OutCmd, Target};

macro_rules! send_log_return {
    ($addr:expr, $text:expr) => {{
        if let Ok(s) = CommandClient::Log(Log($text)).serialize() {
            return Some(vec![OutCmd::Send {
                targets: Target::Some(vec![$addr]),
                msg: s,
            }]);
        } else {
            error!("Couldnt send log to client");
        }
    }};
}

macro_rules! add_log_return {
    ($addr:expr, $text:expr, $vec:expr) => {{
        if let Ok(s) = CommandClient::Log(Log(($text).to_string())).serialize() {
            $vec.push(OutCmd::Send {
                targets: Target::Some(vec![$addr]),
                msg: s,
            });
            return Some($vec);
        } else {
            error!("Couldn't send log to client");
            return Some($vec); // still return, so it always diverges
        }
    }};
}

macro_rules! push_update_province_all_or_return_none {
    ($ret:expr, $world:expr, $idx:expr) => {{
        let __idx_u32 = $idx as u32;

        let __change = CommandClient::UpdateProvince(UpdateProvince {
            id: __idx_u32,
            province: $world.provinces[__idx_u32 as usize].clone(),
        });

        let __msg = match __change.serialize() {
            Ok(x) => x,
            Err(_) => {
                error!("Couldn serialzie {:?}", __change);
                return None;
            }
        };

        $ret.push(OutCmd::Send {
            targets: Target::All,
            msg: __msg,
        });
    }};
}

pub trait ExecuteAttack {
    fn execute(
        &self,
        world: &mut GameWorld,
        id_country: Option<u32>,
        addr: SocketAddr,
    ) -> Option<Vec<OutCmd>>;
}

impl ExecuteAttack for Attack {
    fn execute(
        &self,
        world: &mut GameWorld,
        id_country: Option<u32>,
        addr: SocketAddr,
    ) -> Option<Vec<OutCmd>> {
        let id_country = match id_country {
            None => {
                send_log_return!(addr, "Choose country first".to_string());
                return None;
            }
            Some(x) => x,
        };

        if world.provinces[self.from_province as usize].owner_id != id_country {
            send_log_return!(
                addr,
                "You cant only move army from you own province".to_string()
            );
        }

        if !world.id.adjacency[self.from_province as usize].contains(&self.to_province) {
            send_log_return!(
                addr,
                "You cant only move army to adjacenc provinces".to_string()
            );
        }

        if !world.id.adjacency[self.from_province as usize].contains(&self.to_province) {
            send_log_return!(
                addr,
                "You cant only move army to adjacenc provinces".to_string()
            );
        }

        let mut ret: Vec<OutCmd> = vec![];

        let army = world.provinces[self.from_province as usize].army;
        world.provinces[self.from_province as usize].army = 0;
        let enemy = world.provinces[self.to_province as usize].owner_id;

        push_update_province_all_or_return_none!(ret, world, self.from_province);

        if world.provinces[self.to_province as usize].terrain_type == TERRAIN_WATER {
            add_log_return!(addr, "All of your army drowned".to_string(), ret);
        }

        if enemy == id_country {
            world.provinces[self.from_province as usize].army = 0;
            world.provinces[self.to_province as usize].army = army;
            push_update_province_all_or_return_none!(ret, world, self.to_province);
            send_log_return!(addr, "You have moved your army".to_string());
        }

        if enemy != NO_OWNER && !world.countries[id_country as usize].war.contains(&enemy) {
            push_update_province_all_or_return_none!(ret, world, self.to_province);
            add_log_return!(
                addr,
                "You have to be at war with another country to send there army".to_string(),
                ret
            );
        }

        if enemy == NO_OWNER {
            world.provinces[self.to_province as usize].army = army;
            world.provinces[self.to_province as usize].owner_id = id_country;

            push_update_province_all_or_return_none!(ret, world, self.to_province);
            add_log_return!(
                addr,
                "You have counquerd another province :)".to_string(),
                ret
            );
        } else {
            let enemy_army = world.provinces[self.to_province as usize].army;
            if enemy_army > army {
                world.provinces[self.to_province as usize].army = enemy_army - army;

                push_update_province_all_or_return_none!(ret, world, self.to_province);
                add_log_return!(addr, "All of your solders died".to_string(), ret);
            } else {
                world.provinces[self.to_province as usize].army = army - enemy_army;
                world.provinces[self.to_province as usize].owner_id = id_country;

                push_update_province_all_or_return_none!(ret, world, self.to_province);
                add_log_return!(
                    addr,
                    "You have counquerd another province :)".to_string(),
                    ret
                );
            }
        }

        None
    }
}

pub trait ExecuteChooseCountry {
    fn execute(
        &self,
        world: &mut GameWorld,
        active: &mut ActiveClients,
        addr: SocketAddr,
    ) -> Option<Vec<OutCmd>>;
}

impl ExecuteChooseCountry for ChooseCountry {
    fn execute(
        &self,
        world: &mut GameWorld,
        active: &mut ActiveClients,
        addr: SocketAddr,
    ) -> Option<Vec<OutCmd>> {
        if self.0 == NO_OWNER {
            send_log_return!(addr, "Selected province doesnt have owner".to_string());
        }
        let exists = active.0.values().any(|v| v == &Some(self.0));
        if exists {
            send_log_return!(addr, "Country already taken".to_string());
        }

        active.0.insert(addr, Some(self.0));

        send_log_return!(addr, "Country chosen succesfully".to_string());

        None
    }
}
