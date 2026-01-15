use bevy::{
    camera::Viewport, prelude::*, render::storage::ShaderStorageBuffer,
    sprite_render::Material2dPlugin,
};

use crate::data::{GPUMaterial, GPUMaterialHandle, GameState, GameWorld};

pub struct CreateScenePlugin;

impl Plugin for CreateScenePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GPUMaterialHandle>()
            .init_asset::<GPUMaterial>()
            .add_plugins(Material2dPlugin::<GPUMaterial>::default())
            .add_systems(OnEnter(GameState::Game), (copy_to_gpu, create_scene));
    }
}

fn copy_to_gpu(
    world: Res<GameWorld>,

    mut gpu_materials: ResMut<Assets<GPUMaterial>>,
    gpu_handle: Res<GPUMaterialHandle>,

    mut buffers: ResMut<Assets<ShaderStorageBuffer>>,
) {
    if let Some(gpu) = gpu_materials.get_mut(gpu_handle.0.id()) {
        if let Some(country) = buffers.get_mut(gpu.countries.id()) {
            *country = ShaderStorageBuffer::from(world.countries.clone());
        }
        if let Some(provinces) = buffers.get_mut(gpu.provinces.id()) {
            *provinces = ShaderStorageBuffer::from(world.provinces.clone());
        }
    }
}

fn create_scene(
    mut commands: Commands,
    gpu_handle: Res<GPUMaterialHandle>,
    mut meshes: ResMut<Assets<Mesh>>,

    window: Single<&Window>,
    world: Res<GameWorld>,
) {
    let window_size = window.resolution.physical_size();

    commands.spawn((
        Camera2d,
        Camera {
            viewport: Some(Viewport {
                physical_position: uvec2(0, 0),
                physical_size: window_size,
                ..default()
            }),
            ..default()
        },
    ));

    let rect_handle = meshes.add(Rectangle::from_size(Vec2::new(
        world.width() as f32,
        world.height() as f32,
    )));













    
    commands.spawn((Mesh2d(rect_handle), MeshMaterial2d(gpu_handle.0.clone())));
}
