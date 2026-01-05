use crate::data::GameState;
use bevy::asset::AssetPath;
use bevy::image::ImageSampler;
use bevy::render::render_resource::TextureFormat;
use bevy::tasks::futures_lite::io::AssertAsync;
use bevy::text::TextRoot;
use bevy::{prelude::*, text};
use std::path::Path;

pub struct SetMapPlugin;

use crate::data::*;

impl Plugin for SetMapPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<GPUMap>()
            .add_systems(
                OnEnter(GameState::StartGame),
                ((add_resource, set_map).chain(), loading_screen_setup),
            )
            .add_systems(
                Update,
                (check, configure_sampler).run_if(in_state(GameState::StartGame)),
            );
    }
}

#[derive(Component)]
struct CheckTimer(Timer, bool);

fn loading_screen_setup(mut commands: Commands) {
    commands.spawn((DespawnOnExit(GameState::StartGame), Camera2d::default()));

    commands.spawn((
        DespawnOnExit(GameState::StartGame),
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,

            ..default()
        },
        children![(
            Text::new("Loading"),
            TextFont {
                font_size: 67.0,
                ..default()
            },
            Node {
                margin: UiRect::all(px(50)),
                ..default()
            },
        ),],
    ));

    commands.spawn((
        DespawnOnExit(GameState::StartGame),
        CheckTimer(Timer::from_seconds(1.0, TimerMode::Once), false),
    ));
}

fn check(
    time: Res<Time>,
    mut timer: Single<&mut CheckTimer>,
    mut map: ResMut<Map>,

    asset_server: Res<AssetServer>,
    game_start_type: Res<GameStartType>,
    images: Res<Assets<Image>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if timer.0.tick(time.delta()).is_finished() {
        timer.1 = true;
    }
    if !timer.1 {
        return;
    }
    // todo should wait or sth for image loading
    match &*game_start_type {
        GameStartType::NewGame(data) => {
            let id = asset_server.load(AssetPath::from_path(Path::new(&data.id_path)));
            map.set_map_from_image(&*images, &id);
        }
        _ => {}
    }

    if map.is_ready() {
        println!("ready");
        next_state.set(GameState::Game);
    }
}

// todo should check i f images are loaded and cooperate wiht chekc or sth
fn configure_sampler(
    textures_handle: Res<GPUMapHandle>,
    materials: Res<Assets<GPUMap>>,
    mut images: ResMut<Assets<Image>>,
) {
    let Some(material) = materials.get(&textures_handle.0) else {
        return; // material not created yet
    };

    if let Some(image) = images.get_mut(&material.map_handle) {
        image.sampler = ImageSampler::nearest();
        image.texture_descriptor.format = TextureFormat::Rgba8Unorm;
    }
}

fn add_resource(mut commands: Commands) {
    commands.init_resource::<Map>();
    commands.init_resource::<GPUMapHandle>();
}

fn set_map(
    game_start_type: Res<GameStartType>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,

    mut map: ResMut<Map>,
    mut gpu_maps: ResMut<Assets<GPUMap>>,
    mut gpu_handle: ResMut<GPUMapHandle>,
) {
    match &*game_start_type {
        GameStartType::NewGame(data) => {
            new_game_setup(&data, asset_server, &mut *gpu_maps, gpu_handle, map);
        }
        GameStartType::Load(_) => load_game_setup(),
        // todo error
        GameStartType::Undefined => {}
    }
}

fn new_game_setup(
    data: &GameData,
    asset_server: Res<AssetServer>,

    gpu_maps: &mut Assets<GPUMap>,
    mut gpu_handle: ResMut<GPUMapHandle>,

    mut map: ResMut<Map>,
) {
    println!("new game");
    if data.is_loaded(asset_server) {
        load_images(data, gpu_maps, gpu_handle);
    }
}

fn load_images(
    data: &GameData,
    gpu_maps: &mut Assets<GPUMap>,
    mut gpu_handle: ResMut<GPUMapHandle>,
) {
    let handle = gpu_maps.add(GPUMap::new(data));

    gpu_handle.0 = handle;
}

fn load_game_setup() {
    println!("load game");
}
