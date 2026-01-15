use bevy::prelude::*;
use bevy::{app::AppExit, platform::thread};
use std::time::Duration;
use std::{
    net::TcpStream,
    sync::{Mutex, mpsc},
};

use shared::{commands_server::basic::*, commands_server::command_server::*};

use super::apply::apply;
use super::threads::{spawn_client_reader, spawn_client_writer};
use super::types::{ClientEvent, ClientInbox, ClientOutbox};

pub struct ConnectionPlugin;

impl Plugin for ConnectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, connect_and_spawn_threads)
            .add_systems(PostStartup, apply)
            .add_systems(FixedUpdate, apply)
            .add_systems(Update, send_to_server);
    }
}

fn connect_and_spawn_threads(mut commands: Commands) {
    let stream = match TcpStream::connect("127.0.0.1:7000") {
        Ok(s) => s,
        Err(e) => {
            error!("Failed to connect to server: {e}");
            commands.write_message(AppExit::error());
            return;
        }
    };

    let write_stream = match stream.try_clone() {
        Ok(s) => s,
        Err(e) => {
            error!("Failed to clone stream for writer: {e}");
            commands.write_message(AppExit::error());
            return;
        }
    };

    let (bevy_in, bevy_out) = mpsc::channel::<ClientEvent>();
    let (writer_in, writer_out) = mpsc::channel::<String>();

    commands.insert_resource(ClientInbox(Mutex::new(bevy_out)));
    commands.insert_resource(ClientOutbox(writer_in));

    spawn_client_reader(stream, bevy_in);
    spawn_client_writer(write_stream, writer_out);

    info!("Client connected and IO threads started.");
}

fn send_to_server(outbox: Res<ClientOutbox>) {

    // let cmd = CommandServer::UpgradeProvince(UpgradeProvince {});
    // let msg = match cmd.serialize() {
    //     Ok(x) => x,
    //     Err(e) => {
    //         error!("Couldnt serialize [{cmd:?}] error: [{e}]");
    //         return;
    //     }
    // };

    // if let Err(e) = outbox.0.send(msg.to_string()) {
    //     error!("Failed to queue message to server: {e}");
    // }
}
