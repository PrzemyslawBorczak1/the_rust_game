use super::super::ui::{GPUMaterial, GPUMaterialHandle};
use crate::ui::interface::MessageLog;
use crate::ui::interface::common::Refresch;
use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::render::storage::ShaderStorageBuffer;
use shared::commands_client::CommandClient;
use shared::resources::GameWorld;
use std::sync::mpsc::TryRecvError;

use crate::cmd_impls::Execute;

use super::types::*;

pub fn apply(
    inbox: Res<ClientInbox>,
    mut commands: Commands,
    mut world: ResMut<GameWorld>,
    mut gpu_materials: ResMut<Assets<GPUMaterial>>,
    mut handle: ResMut<GPUMaterialHandle>,
    mut buffers: ResMut<Assets<ShaderStorageBuffer>>,
    mut meshes: ResMut<Assets<Mesh>>,
    q_root: Query<&mut Text, With<MessageLog>>,
    mut writer: MessageWriter<Refresch>,
) {
    let bevy_in = match inbox.0.lock() {
        Ok(g) => g,
        Err(poisoned) => {
            error!("ClientInbox mutex poisoned: {poisoned}");
            commands.write_message(AppExit::error());
            return;
        }
    };
    for mut text in q_root {
        loop {
            match bevy_in.try_recv() {
                Ok(ClientEvent::Line(line)) => match CommandClient::deserialize(&line) {
                    Ok(cmd) => {
                        cmd.execute(
                            &mut world,
                            &mut commands,
                            gpu_materials.get_mut(&handle.0),
                            &mut handle,
                            &mut *buffers,
                            &mut text,
                            &mut meshes,
                            &mut writer,
                        );
                    }
                    Err(e) => {
                        warn!("Failed to parse JSON: {e}. Line: {line}");
                    }
                },
                Ok(ClientEvent::Error(msg)) => {
                    error!("Server-side/client read error: {msg}");
                    commands.write_message(AppExit::error());
                    return;
                }
                Ok(ClientEvent::Disconnected) => {
                    error!("Server disconnected.");
                    commands.write_message(AppExit::error());
                    return;
                }
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => {
                    error!("Client inbox channel disconnected.");
                    commands.write_message(AppExit::error());
                    return;
                }
            }
        }
    }
}
