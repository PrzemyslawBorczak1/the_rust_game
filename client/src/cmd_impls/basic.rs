use crate::ui::{CountryGpu, ProvinceGpu};

use super::super::ui::{GPUMaterial, GPUMaterialHandle};
use super::command_client::*;
use bevy::prelude::*;
use bevy::render::render_resource::encase::StorageBuffer;
use bevy::render::storage::ShaderStorageBuffer;
use shared::{commands_client::basic::*, resources::GameWorld};

pub trait ExecuteInit {
    fn execute(
        self,
        world: &mut GameWorld,
        commands: &mut Commands,
        gpu_materials: Option<&mut GPUMaterial>,
        handle: &mut GPUMaterialHandle,
        buffers: &mut Assets<ShaderStorageBuffer>,
        meshes: &mut Assets<Mesh>,
    );
}

impl ExecuteInit for Init {
    fn execute(
        self,
        world: &mut GameWorld,
        commands: &mut Commands,
        gpu_materials: Option<&mut GPUMaterial>,
        handle: &mut GPUMaterialHandle,
        buffers: &mut Assets<ShaderStorageBuffer>,
        meshes: &mut Assets<Mesh>,
    ) {
        *world = self.world;
        info!["{:?}", world.id.map.len()];

        if let Some(gpu) = gpu_materials {
            gpu.width = world.id.width;
            gpu.height = world.id.height;

            gpu.id = buffers.add(ShaderStorageBuffer::from(world.id.map.clone()));
            gpu.provinces = buffers.add(ShaderStorageBuffer::from(
                world
                    .provinces
                    .iter()
                    .map(|p| ProvinceGpu {
                        owner_id: p.owner_id,
                        terrain_type: p.terrain_type,
                    })
                    .collect::<Vec<ProvinceGpu>>(),
            ));
            gpu.countries = buffers.add(ShaderStorageBuffer::from(
                world
                    .countries
                    .iter()
                    .map(|c| CountryGpu { color: c.color })
                    .collect::<Vec<CountryGpu>>(),
            ));
        }
        let rect_handle = meshes.add(Rectangle::from_size(Vec2::new(
            world.width() as f32,
            world.height() as f32,
        )));

        commands.spawn((Mesh2d(rect_handle), MeshMaterial2d(handle.0.clone())));

        let mut v = vec![];
        world.id.map.iter().for_each(|val| {
            if !v.contains(val) {
                v.push(*val);
            }
        });
        info!["vals: {v:?}"];
        info!["vals: {:#?}", world.countries];
    }
}

pub trait ExecuteLog {
    fn execute(self, logger: &mut Text);
}

impl ExecuteLog for Log {
    fn execute(self, logger: &mut Text) {
        logger.0 = self.0;
    }
}

pub trait ExecuteUpdateProvince {
    fn execute(self,
        world: &mut GameWorld,
        gpu_materials: Option<&mut GPUMaterial>,
        buffers: &mut Assets<ShaderStorageBuffer>,
);
}

impl ExecuteUpdateProvince for UpdateProvince {
    fn execute(self,
            world: &mut GameWorld,
            gpu_materials: Option<&mut GPUMaterial>,
            buffers: &mut Assets<ShaderStorageBuffer>,
    ) {
        world.provinces[self.id as usize] = self.province;

        if let Some(material) = gpu_materials{
            material.provinces = buffers.add(ShaderStorageBuffer::from(
                world
                    .provinces
                    .iter()
                    .map(|p| ProvinceGpu {
                        owner_id: p.owner_id,
                        terrain_type: p.terrain_type,
                    })
                    .collect::<Vec<ProvinceGpu>>(),
            ));
        }
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
