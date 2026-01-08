use crate::data::{
    FetchGamePath, FetchHandles, GameState, IdMap, IdMapHandle, Province, ProvinceLoadingState,
    Textures, TexturesHandle, VecCountry, VecCountryHandle, VecProvince, VecProvinceHandle,
    loaders::*,
};
use bevy::log::error;

use anyhow::{Context, Result};
use bevy::camera::Viewport;
use bevy::render::render_resource::{AsBindGroup, ShaderType};
use bevy::render::storage::ShaderStorageBuffer;
use bevy::shader::ShaderRef;
use bevy::sprite_render::{Material2d, Material2dPlugin};
use bevy::{asset, prelude::*};
use bevy::{prelude::*, reflect::TypePath};

pub struct LoadAssetsPlugin;
impl Plugin for LoadAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<ProvinceLoadingState>()
            .init_asset::<VecProvince>()
            .init_resource::<VecProvinceHandle>()
            .add_systems(
                OnEnter(GameState::LoadGame),
                load_province.pipe(loading_error),
            )
            .add_systems(Update, consolidate_province)
            .init_asset::<VecCountry>()
            .init_resource::<VecCountryHandle>()
            .add_systems(
                OnEnter(GameState::LoadGame),
                load_country.pipe(loading_error),
            )
            .init_asset::<IdMap>()
            .init_resource::<IdMapHandle>()
            .init_asset_loader::<IdMapLoader>()
            .add_systems(OnEnter(GameState::LoadGame), load_id_map)
            .init_asset::<Textures>()
            .init_resource::<TexturesHandle>()
            .add_systems(
                OnEnter(GameState::LoadGame),
                load_texture.pipe(loading_error),
            );
    }
}

fn loading_error(In(result): In<Result<()>>) {
    match result {
        Ok(_) => {}
        Err(e) => {
            error!("Loading error occurred: {:#}", e);
        }
    }
}

fn load_province(
    mut assets: ResMut<Assets<VecProvince>>,
    mut handle: ResMut<VecProvinceHandle>,
    path: Res<FetchGamePath>,
) -> Result<()> {
    let asset = VecProvince::deserialize(&path.vec_provinces)
        .with_context(|| format!("[{}]", &path.vec_provinces))?;
    handle.0 = assets.add(asset);

    Ok(())
}

fn consolidate_province() {}

fn load_country(
    mut assets: ResMut<Assets<VecCountry>>,
    mut handle: ResMut<VecCountryHandle>,
    path: Res<FetchGamePath>,
) -> Result<()> {
    let asset = VecCountry::deserialize(&path.vec_country)
        .with_context(|| format!("[{}]", &path.vec_country))?;
    handle.0 = assets.add(asset);

    Ok(())
}

fn load_id_map(
    mut handle: ResMut<IdMapHandle>,
    path: Res<FetchGamePath>,
    asset_server: Res<AssetServer>,
) {
    let asset = asset_server.load::<IdMap>(&path.id_texture);
    handle.0 = asset;
}

fn load_texture(
    mut assets: ResMut<Assets<Textures>>,
    mut handle: ResMut<TexturesHandle>,
    path: Res<FetchGamePath>,
) -> Result<()> {
    let asset = Textures::from(&path.id_texture)?;
    handle.0 = assets.add(asset);
    Ok(())
}

fn consolidate(
    prov: Res<Assets<VecProvince>>,
    mut prov_handle: ResMut<VecProvinceHandle>,

    country: Res<Assets<VecCountry>>,
    mut country_handle: ResMut<VecCountryHandle>,

    texture: Mut<Assets<Textures>>,
    mut texture_handle: ResMut<TexturesHandle>,

    mut map: ResMut<Assets<IdMap>>,
    mut map_handle: ResMut<IdMapHandle>,

    mut commands: Commands,
    mut fetch_handles: FetchHandles,
) {
}

// todo delete

#[derive(Clone, ShaderType)]
struct test {
    a: u32,
    b: test2,
}

#[derive(Clone, ShaderType)]
struct test2 {
    a: u32,
    b: u32,
}

#[derive(AsBindGroup, Debug, Clone, Asset, TypePath, Default)]
struct GPUTest {
    #[texture(1)]
    #[sampler(2)]
    pub texture: Handle<Image>,

    #[storage(3, read_only)]
    pub id: Handle<ShaderStorageBuffer>,
}

impl Material2d for GPUTest {
    fn fragment_shader() -> ShaderRef {
        "shaders/test.wgsl".into()
    }
}

#[derive(Debug, Resource, Default)]
struct GPUTestHandler {
    material: Handle<GPUTest>,
    spawned: bool,
}

fn add_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    window: Single<&Window>,
    handles: Res<FetchHandles>,
    id_maps: Res<Assets<IdMap>>,
    mut materials: ResMut<Assets<GPUTest>>,
    mut gpu_h: ResMut<GPUTestHandler>,
    mut buffers: ResMut<Assets<ShaderStorageBuffer>>,
) {
    if gpu_h.spawned {
        return;
    }

    // Wait until IdMap is loaded (otherwise there's often no camera spawned yet).
    if id_maps.get(&handles.id_map).is_none() {
        return;
    }

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

    let rect_handle = meshes.add(Rectangle::from_size(Vec2::new(1000.0, 600.0)));

    // Start with values that will NOT hit (id < 5) so you see the texture.
    let initial: Vec<test> = vec![
        test {
            a: 1,
            b: test2 { a: 1, b: 1 }
        };
        9
    ];
    let material = materials.add(GPUTest {
        texture: handles.province_texture.clone(),
        id: buffers.add(ShaderStorageBuffer::from(initial)),
    });

    commands.spawn((Mesh2d(rect_handle), MeshMaterial2d(material.clone())));

    gpu_h.material = material;
    gpu_h.spawned = true;
}

fn change(
    gpu_h: Res<GPUTestHandler>,
    mut materials: ResMut<Assets<GPUTest>>,
    mut buffers: ResMut<Assets<ShaderStorageBuffer>>,
    time: Res<Time>,
    mut timer: Local<Timer>,
    mut state: Local<bool>,
) {
    if !gpu_h.spawned {
        return;
    }

    if timer.duration().is_zero() {
        *timer = Timer::from_seconds(0.5, TimerMode::Repeating);
    }
    timer.tick(time.delta());
    if !timer.just_finished() {
        return;
    }

    let Some(material) = materials.get_mut(&gpu_h.material) else {
        return;
    };

    let Some(buf) = buffers.get_mut(&material.id) else {
        return;
    };

    let v: Vec<test> = if *state {
        vec![
            test {
                a: 8,
                b: test2 { a: 2, b: 8 }
            };
            10
        ]
    } else {
        vec![
            test {
                a: 8,
                b: test2 { a: 3, b: 8 }
            };
            10
        ]
    };
    *state = !*state;

    *buf = ShaderStorageBuffer::from(v);
}
