use bevy::prelude::*;
use std::{
    io::{BufRead, BufReader, Write},
    net::TcpStream,
    sync::mpsc::{Receiver, Sender},
    thread,
};

use super::types::ClientEvent;

pub fn spawn_client_reader(stream: TcpStream, bevy_in: Sender<ClientEvent>) {
    thread::spawn(move || {
        let mut reader = BufReader::new(stream);

        loop {
            let mut line = String::new();
            match reader.read_line(&mut line) {
                Ok(0) => {
                    let _ = bevy_in.send(ClientEvent::Disconnected);
                    break;
                }
                Ok(_) => {
                    let line = line.trim().to_string();
                    if !line.is_empty() {
                        if let Err(e) = bevy_in.send(ClientEvent::Line(line)) {
                            error!("Client reader: failed to send Line event: {e}");
                            break;
                        }
                    }
                }
                Err(e) => {
                    let _ = bevy_in.send(ClientEvent::Error(format!("read error: {e}")));
                    break;
                }
            }
        }
    });
}

pub fn spawn_client_writer(mut stream: TcpStream, writer_out: Receiver<String>) {
    thread::spawn(move || {
        while let Ok(msg) = writer_out.recv() {
            if let Err(e) = stream.write_all(msg.as_bytes()) {
                error!("Client writer: write_all failed: {e}");
                break;
            }
            if let Err(e) = stream.write_all(b"\n") {
                error!("Client writer: write newline failed: {e}");
                break;
            }
            if let Err(e) = stream.flush() {
                error!("Client writer: flush failed: {e}");
                break;
            }
        }
    });
}
