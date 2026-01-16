use std::net::SocketAddr;

use bevy::prelude::*;

use shared::{
    commands_client::{CommandClient, basic::*},
    commands_server::basic::*,
    resources::{GameWorld, NO_OWNER, TERRAIN_WATER},
};

use crate::game::net::types::{ActiveClients, OutCmd, Target};

fn make_send(targets: Target, cmd: CommandClient) -> Option<OutCmd> {
    let msg = match cmd.serialize() {
        Ok(s) => s,
        Err(e) => {
            error!("Couldn't serialize {:?}: {:?}", cmd, e);
            return None;
        }
    };

    Some(OutCmd::Send { targets, msg })
}

pub fn log_only(addr: SocketAddr, text: impl Into<String>) -> Option<Vec<OutCmd>> {
    let cmd = CommandClient::Log(Log(text.into()));
    Some(vec![make_send(Target::Some(vec![addr]), cmd)?])
}

pub fn append_log(
    addr: SocketAddr,
    text: impl Into<String>,
    mut out: Vec<OutCmd>,
) -> Option<Vec<OutCmd>> {
    let cmd = CommandClient::Log(Log(text.into()));
    if let Some(send) = make_send(Target::Some(vec![addr]), cmd) {
        out.push(send);
    }
    Some(out)
}

pub fn push_cmd_all(out: &mut Vec<OutCmd>, cmd: CommandClient) -> Option<()> {
    out.push(make_send(Target::All, cmd)?);
    Some(())
}

pub fn push_update_province_all(out: &mut Vec<OutCmd>, world: &GameWorld, idx: u32) -> Option<()> {
    let cmd = CommandClient::UpdateProvince(UpdateProvince {
        id: idx,
        province: world.provinces[idx as usize].clone(),
    });
    push_cmd_all(out, cmd)
}

pub fn require_selected_country(
    active: &ActiveClients,
    addr: SocketAddr,
) -> Result<u32, Vec<OutCmd>> {
    match active.0.get(&addr).copied().flatten() {
        Some(id) => Ok(id),
        None => Err(log_only(addr, "Choose country first".to_string()).unwrap_or_default()),
    }
}
