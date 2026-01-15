use super::super::command_impl::command::Execute;
use bevy::prelude::*;
use shared::{
    commands_client::{CommandClient, basic::Init}, commands_server::CommandServer, resources::GameWorld,
};
use std::sync::mpsc::TryRecvError;

use crate::game::net::types::Target;

use super::types::{ActiveClients, InEvent, NetInbox, NetOutbox, OutCmd};

pub fn apply_net_events(
    bevy_out: Res<NetInbox>,
    writer_in: Res<NetOutbox>,
    mut active: ResMut<ActiveClients>,
    mut world: ResMut<GameWorld>,
) {
    let rx = match bevy_out.0.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            error!("NetInbox mutex poisoned: {poisoned}");
            return;
        }
    };

    loop {
        match rx.try_recv() {
            Ok(ev) => {
                if !handle_event(ev, &writer_in, &mut active, &mut world) {
                    break;
                }
            }

            Err(TryRecvError::Empty) => break,

            Err(TryRecvError::Disconnected) => {
                warn!("incoming channel disconnected (all senders dropped)");
                break;
            }
        }
    }
}

fn handle_event(
    ev: InEvent,
    outbox: &Res<NetOutbox>,
    active: &mut ResMut<ActiveClients>,
    world: &mut ResMut<GameWorld>,
) -> bool {
    match ev {
        InEvent::Connected { addr } => handle_connected(addr, outbox, active, world),
        InEvent::Disconnected { addr } => {
            handle_disconnected(addr, outbox, active);
            true
        }
        InEvent::Command { addr, command } => {
            handle_command(addr, command, outbox, world);
            true
        }
        InEvent::Error { addr, msg } => {
            warn!("net error {:?}: {}", addr, msg);
            true
        }
    }
}

fn handle_connected(
    addr: std::net::SocketAddr,
    outbox: &Res<NetOutbox>,
    active: &mut ResMut<ActiveClients>,
    world: &mut ResMut<GameWorld>,
) -> bool {
    info!("client connected: {addr}");
    active.set.insert(addr);

    let cmd = CommandClient::Init(Init{
        world: (**world).clone()
    });

    let world_serialized = match cmd.serialize() {
        Ok(s) => s,
        Err(e) => {
            error!("Failed to serialize GameWorld: {e}");
            return true;
        }
    };

    if let Err(e) = outbox.0.send(OutCmd::Send {
        targets: Target::Some(vec![addr]),
        msg: world_serialized,
    }) {
        error!("[{addr}] failed to enqueue initial world: {e}");
    }

    true
}

fn handle_disconnected(
    addr: std::net::SocketAddr,
    outbox: &Res<NetOutbox>,
    active: &mut ResMut<ActiveClients>,
) {
    info!("client disconnected: {addr}");
    active.set.remove(&addr);

    if let Err(e) = outbox.0.send(OutCmd::RemoveClient { addr }) {
        error!("[{addr}] failed to enqueue RemoveClient: {e}");
    }
}

fn handle_command(
    addr: std::net::SocketAddr,
    command: CommandServer,
    outbox: &Res<NetOutbox>,
    world: &mut GameWorld,
) {
    info!("from {addr}: {command:#?}");

    let commands = match command.execute(world) {
        Some(ch) => ch,
        None => return,
    };

    for command in commands {
        if let Err(e) = outbox.0.send(command) {
            error!("[{addr}] failed to enqueue echo: {e}");
        }
    }
}
