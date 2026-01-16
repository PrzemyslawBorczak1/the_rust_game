use bevy::prelude::*;
use shared::{commands_client::CommandClient, commands_server::CommandServer};
use std::{
    collections::{HashMap, HashSet},
    net::{SocketAddr, TcpStream},
    sync::{
        Mutex,
        mpsc::{Receiver, Sender},
    },
};

/// client --> bevy
#[derive(Debug)]
pub enum InEvent {
    Connected {
        addr: SocketAddr,
    },
    Disconnected {
        addr: SocketAddr,
    },
    Command {
        addr: SocketAddr,
        command: CommandServer,
    },
    Error {
        addr: Option<SocketAddr>,
        msg: String,
    },
}

/// client --> bevy
#[derive(Resource)]
pub struct NetInbox(pub Mutex<Receiver<InEvent>>);

/// bevy --> client writer
#[derive(Debug)]
pub enum OutCmd {
    AddClient { addr: SocketAddr, stream: TcpStream },
    RemoveClient { addr: SocketAddr },

    Send { targets: Target, msg: String },
}



#[derive(Debug)]
pub enum Target {
    All,
    Some(Vec<SocketAddr>),
}

/// bevy --> client writer
#[derive(Resource)]
pub struct NetOutbox(pub Sender<OutCmd>);

// for writer
#[derive(Resource, Default)]
pub struct ActiveClients(pub HashMap<SocketAddr, Option<u32>>);
