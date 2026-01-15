use bevy::prelude::*;
use std::sync::{
    mpsc::{Receiver, Sender},
    Mutex,
};

#[derive(Debug)]
pub enum ClientEvent {
    Line(String),
    Error(String),
    Disconnected,
}

#[derive(Resource)]
pub struct ClientInbox(pub Mutex<Receiver<ClientEvent>>);

#[derive(Resource)]
pub struct ClientOutbox(pub Sender<String>);
