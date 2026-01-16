use bevy::{prelude::*, render::storage::ShaderStorageBuffer, sprite_render::Material2dPlugin};

use shared::{gpu::*, resources::GameWorld};

use crate::{data::GameState, history};

pub struct HistoryGraphics;

impl Plugin for HistoryGraphics {
    fn build(&self, app: &mut App) {
        app.init_resource::<GPUMaterialHandle>()
            .init_asset::<GPUMaterial>()
            .add_plugins(Material2dPlugin::<GPUMaterial>::default())
            .add_systems(OnEnter(GameState::History), add_gpu_material);
    }
}

fn add_gpu_material(
    mut gpu_materials: ResMut<Assets<GPUMaterial>>,
    mut gpu_handle: ResMut<GPUMaterialHandle>,
    world: Res<GameWorld>,
    mut buffers: ResMut<Assets<ShaderStorageBuffer>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
) {
    println!("dfgh");

    let mut gpu = GPUMaterial::default();
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

    let rect_handle = meshes.add(Rectangle::from_size(Vec2::new(
        world.width() as f32,
        world.height() as f32,
    )));

    gpu_handle.0 = gpu_materials.add(gpu);

    commands.spawn((Mesh2d(rect_handle), MeshMaterial2d(gpu_handle.0.clone())));
}
