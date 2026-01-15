use super::super::ui::{GPUMaterial, GPUMaterialHandle};
use bevy::{prelude::*, render::storage::ShaderStorageBuffer};
use shared::{commands_client::CommandClient, resources::GameWorld};

pub trait Execute {
    fn execute(
        self,
        world: &mut GameWorld,
        commands: &mut Commands,
        gpu_materials: Option<&mut GPUMaterial>,
        handle: &mut Handle<GPUMaterial>,
        buffers: &mut Assets<ShaderStorageBuffer>,
        meshes: &mut Assets<Mesh>,
    );
}

impl Execute for CommandClient {
    fn execute(
        self,
        world: &mut GameWorld,
        commands: &mut Commands,
        gpu_materials: Option<&mut GPUMaterial>,
        handle: &mut Handle<GPUMaterial>,
        buffers: &mut Assets<ShaderStorageBuffer>,

        meshes: &mut Assets<Mesh>,
    ) {
        match self {
            // CommandClient::ChangeCountry(cmd) => {
            //     cmd.execute(world, commands, gpu_materials, handle, buffers, meshes)
            // }
            // CommandClient::ChangeProvince(cmd) => {
            //     cmd.execute(world, commands, gpu_materials, handle, buffers, meshes)
            // }
            CommandClient::Init(cmd) => {
                cmd.execute(world, commands, gpu_materials, handle, buffers, meshes)
            }
            _ => {}
        }
    }
}
