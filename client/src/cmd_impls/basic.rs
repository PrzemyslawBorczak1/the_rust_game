use super::super::ui::{GPUMaterial, GPUMaterialHandle};
use super::command_client::*;
use bevy::prelude::*;
use bevy::render::storage::ShaderStorageBuffer;
use shared::{commands_client::basic::*, resources::GameWorld};

impl Execute for Init {
    fn execute(
        self,
        world: &mut GameWorld,
        commands: &mut Commands,
        gpu_materials: Option<&mut GPUMaterial>,
        handle: &mut Handle<GPUMaterial>,
        buffers: &mut Assets<ShaderStorageBuffer>,
        meshes: &mut Assets<Mesh>,
    ) {
        *world = self.world;

        if let Some(gpu) = gpu_materials {
            print!("set");

            gpu.width = world.id.width;
            gpu.height = world.id.height;

            gpu.id = buffers.add(ShaderStorageBuffer::from(world.id.map.clone()));
            gpu.provinces = buffers.add(ShaderStorageBuffer::from(world.provinces.clone()));
            gpu.countries = buffers.add(ShaderStorageBuffer::from(world.countries.clone()));
        }
        let rect_handle = meshes.add(Rectangle::from_size(Vec2::new(
            world.width() as f32,
            world.height() as f32,
        )));

        commands.spawn((Mesh2d(rect_handle), MeshMaterial2d(handle.clone())));
    }
}

// impl Execute for Vec<ChangeCountry> {
//     fn execute(
//         &self,
//         world: &mut GameWorld,
//         commands: &mut Commands,
//         gpu_materials: &mut Assets<GPUMaterial>,
//         handle: &mut GPUMaterialHandle,
//         buffers: &mut Assets<ShaderStorageBuffer>,
//         meshes: &mut Assets<Mesh>,
//     ) {
//         println!("Change country");
//     }
// }

// impl Execute for Vec<ChangeProvince> {
//     fn execute(
//         &self,
//         world: &mut GameWorld,
//         commands: &mut Commands,
//         gpu_materials: &mut Assets<GPUMaterial>,
//         handle: &mut GPUMaterialHandle,
//         buffers: &mut Assets<ShaderStorageBuffer>,
//         meshes: &mut Assets<Mesh>,
//     ) {
//         println!("Change Province")
//     }
// }
