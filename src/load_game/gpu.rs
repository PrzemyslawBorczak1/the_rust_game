use bevy::{prelude::*, render::storage::ShaderStorageBuffer};

use crate::{
    data::{GPUMaterial, GPUMaterialHandle, GameWorld},
    load_game::finish::LoadingState,
};

pub struct AddGPUPlugin;

impl Plugin for AddGPUPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GPUMaterialHandle>()
            .init_asset::<GPUMaterial>()
            .add_systems(OnEnter(LoadingState::Finished), add_gpu);
    }
}

fn add_gpu(
    world: Res<GameWorld>,

    mut gpu_materials: ResMut<Assets<GPUMaterial>>,
    mut gpu_handle: ResMut<GPUMaterialHandle>,

    mut buffers: ResMut<Assets<ShaderStorageBuffer>>,
) {
    let material = GPUMaterial {
        id: buffers.add(ShaderStorageBuffer::from(world.id.map.clone())),
        width: world.id.width,
        height: world.id.height,
        provinces: buffers.add(ShaderStorageBuffer::from(vec![0])),
        countries: buffers.add(ShaderStorageBuffer::from(vec![0])),
        selected_id: 0,
        draw_type: 0,
    };

    gpu_handle.0 = gpu_materials.add(material);
}
