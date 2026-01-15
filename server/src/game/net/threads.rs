use bevy::prelude::*;
use shared::commands_server::CommandServer;
use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Write},
    net::{SocketAddr, TcpListener, TcpStream},
    sync::mpsc::{Receiver, Sender},
    thread,
};

use crate::game::net::types::Target;

use super::types::{InEvent, OutCmd};
// in ma pisac ze streama
// out ma czytac ze streama

// bevy kolejka z kotrej czyta system wykonujacy zdarzenia
// writer kolejka z ktorej wysylane jest do clientow
pub fn start_accept_thread(bevy_in: Sender<InEvent>, writer_in: Sender<OutCmd>) {
    thread::spawn(move || {
        let listener = match TcpListener::bind("127.0.0.1:7000") {
            Ok(l) => l,
            Err(e) => {
                error!("Accepting thread bind failed: {e}");
                return;
            }
        };

        accept_loop(listener, bevy_in, writer_in);
    });
}

fn accept_loop(listener: TcpListener, bevy_in: Sender<InEvent>, writer_in: Sender<OutCmd>) {
    loop {
        match listener.accept() {
            Ok((stream, addr)) => {
                handle_accepted_client(stream, addr, &bevy_in, &writer_in);
            }
            Err(e) => {
                error!("accept failed: {e}");
            }
        }
    }
}

fn handle_accepted_client(
    stream: TcpStream,
    addr: SocketAddr,
    bevy_in: &Sender<InEvent>,
    writer_in: &Sender<OutCmd>,
) {
    let read_stream = match stream.try_clone() {
        Ok(s) => s,
        Err(e) => {
            error!("[{addr}] try_clone failed: {e}");
            return;
        }
    };

    if let Err(e) = writer_in.send(OutCmd::AddClient { addr, stream }) {
        error!("[{addr}] failed to send AddClient to writer thread: {e}");
        return;
    }

    if let Err(e) = bevy_in.send(InEvent::Connected { addr }) {
        error!("[{addr}] failed to send Connected event to Bevy: {e}");
        return;
    }

    spawn_client_reader(read_stream, addr, bevy_in.clone());
}

pub fn spawn_client_reader(stream: TcpStream, addr: SocketAddr, bevy_in: Sender<InEvent>) {
    thread::spawn(move || {
        let mut reader = BufReader::new(stream);

        loop {
            if !read_line(&mut reader, addr, &bevy_in) {
                break;
            }
        }

        if let Err(e) = bevy_in.send(InEvent::Disconnected { addr }) {
            error!("[{addr}] failed to send Disconnected event: {e}");
        }
    });
}

fn read_line(
    reader: &mut BufReader<TcpStream>,
    addr: SocketAddr,
    bevy_in: &Sender<InEvent>,
) -> bool {
    let mut line = String::new();
    match reader.read_line(&mut line) {
        Ok(0) => false,
        Ok(_) => {
            let text = line.trim().to_string();
            if text.is_empty() {
                return true;
            }
            let command = match CommandServer::deserialize(&text) {
                Ok(c) => c,
                Err(e) => {
                    if let Err(e) = bevy_in.send(InEvent::Error {
                        addr: Some(addr),
                        msg: format!("Couldnt deserialize [{:?}]", &text),
                    }) {
                        error!("[{addr}] failed to send Line event: {e}");
                        return false;
                    };
                    return true;
                }
            };

            if let Err(e) = bevy_in.send(InEvent::Command { addr, command }) {
                error!("[{addr}] failed to send Line event: {e}");
                return false;
            }
            true
        }
        Err(e) => {
            error!("[{addr}] read error: {e}");
            let _ = bevy_in.send(InEvent::Error {
                addr: Some(addr),
                msg: format!("read error: {e}"),
            });
            false
        }
    }
}

pub fn start_writer_thread(writer_out: Receiver<OutCmd>) {
    thread::spawn(move || {
        let mut clients: HashMap<SocketAddr, TcpStream> = HashMap::new();

        while let Ok(cmd) = writer_out.recv() {
            process_writer_cmd(cmd, &mut clients);
        }

        warn!("[writer] outbox channel closed; writer thread exiting");
    });
}

fn process_writer_cmd(cmd: OutCmd, clients: &mut HashMap<SocketAddr, TcpStream>) {
    match cmd {
        OutCmd::AddClient { addr, stream } => {
            clients.insert(addr, stream);
            info!("[writer] added client {addr}");
        }

        OutCmd::RemoveClient { addr } => {
            clients.remove(&addr);
            info!("[writer] removed client {addr}");
        }

        OutCmd::Send { targets, msg } => match targets {
            Target::All => {
                for (addr, stream) in clients.iter_mut() {
                    send(&addr, Some(stream), &msg);
                }
            }
            Target::Some(addreses) => {
                for addr in addreses {
                    send(&addr, clients.get_mut(&addr), &msg);
                }
            }
        },
    }
}

fn send(addr: &SocketAddr, stream: Option<&mut TcpStream>, msg: &String) {
    match stream {
        Some(stream) => {
            if let Err(e) = stream.write_all(msg.as_bytes()) {
                error!("[writer:{addr}] write_all failed: {e}");
                return;
            }
            if let Err(e) = stream.write_all(b"\n") {
                error!("[writer:{addr}] write newline failed: {e}");
                return;
            }
            if let Err(e) = stream.flush() {
                error!("[writer:{addr}] flush failed: {e}");
            }
        }
        None => {
            warn!("[{addr}] Unknown addres");
        }
    }
}
