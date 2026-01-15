use bevy::prelude::*;
use std::sync::mpsc;

use super::apply::apply_net_events;
use super::threads::{start_accept_thread, start_writer_thread};
use super::types::{ActiveClients, InEvent, NetInbox, NetOutbox, OutCmd};

use crate::data::GameState;

pub struct NetPlugin;

impl Plugin for NetPlugin {
    fn build(&self, app: &mut App) {
        let (in_tx, in_rx) = mpsc::channel::<InEvent>();
        let (out_tx, out_rx) = mpsc::channel::<OutCmd>();

        app.insert_resource(NetInbox(std::sync::Mutex::new(in_rx)))
            .insert_resource(NetOutbox(out_tx.clone()))
            .init_resource::<ActiveClients>();

        start_writer_thread(out_rx);

        let in_tx2 = in_tx.clone();
        let out_tx2 = out_tx.clone();

        app.add_systems(OnEnter(GameState::Game), move || {
            start_accept_thread(in_tx2.clone(), out_tx2.clone());
        })
        .add_systems(Update, apply_net_events.run_if(in_state(GameState::Game)));
    }
}
