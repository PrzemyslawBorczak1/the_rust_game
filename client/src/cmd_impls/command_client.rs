use crate::{
    cmd_impls::basic::{ExecuteInit, ExecuteLog, ExecuteUpdateCountries, ExecuteUpdateProvince},
    ui::interface::common::Refresch,
};

use super::super::ui::{GPUMaterial, GPUMaterialHandle};
use bevy::{prelude::*, render::storage::ShaderStorageBuffer};
use shared::{commands_client::CommandClient, resources::GameWorld};

pub trait Execute {
    fn execute(
        self,
        world: &mut GameWorld,
        commands: &mut Commands,
        gpu_materials: Option<&mut GPUMaterial>,
        handle: &mut GPUMaterialHandle,
        buffers: &mut Assets<ShaderStorageBuffer>,

        logger: &mut Text,
        meshes: &mut Assets<Mesh>,
        writer: &mut MessageWriter<Refresch>,
    );
}

impl Execute for CommandClient {
    fn execute(
        self,
        world: &mut GameWorld,
        commands: &mut Commands,
        gpu_materials: Option<&mut GPUMaterial>,
        handle: &mut GPUMaterialHandle,
        buffers: &mut Assets<ShaderStorageBuffer>,

        logger: &mut Text,
        meshes: &mut Assets<Mesh>,
        writer: &mut MessageWriter<Refresch>,
    ) {
        match self {
            CommandClient::UpdateProvince(cmd) => cmd.execute(world, gpu_materials, buffers),
            CommandClient::Init(cmd) => {
                cmd.execute(world, commands, gpu_materials, handle, buffers, meshes)
            }
            CommandClient::Log(log) => log.execute(logger),
            CommandClient::UpdateCountries(vec) => vec.execute(world, writer),
        }
    }
}
