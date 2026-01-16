use bevy::camera::Viewport;
use bevy::{prelude::*, sprite_render::Material2dPlugin};
use shared::resources::GameWorld;

use super::gpu::{GPUMaterial, GPUMaterialHandle};

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GPUMaterialHandle>()
            .init_asset::<GPUMaterial>()
            .add_plugins(Material2dPlugin::<GPUMaterial>::default())
            .add_systems(Startup, create_scene)
            .add_systems(Startup, add_empty_gpu_material);
    }
}

fn add_empty_gpu_material(
    mut gpu_materials: ResMut<Assets<GPUMaterial>>,
    mut gpu_handle: ResMut<GPUMaterialHandle>,
) {
    let material = GPUMaterial::default();

    gpu_handle.0 = gpu_materials.add(material);
}

fn create_scene(mut commands: Commands, window: Single<&Window>, world: Res<GameWorld>) {
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
        Projection::Orthographic(OrthographicProjection {
            scale: 1.0,
            ..OrthographicProjection::default_2d()
        }),
    ));
}
