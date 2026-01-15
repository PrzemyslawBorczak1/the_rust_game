use crate::data::{FetchGamePath, GameState};

use shared::resources::IdMap;

use bevy::{
    asset::{AssetLoader, LoadContext, io::Reader},
    prelude::*,
};
use thiserror::Error;

use super::resources::*;

#[derive(States, Debug, Hash, Eq, PartialEq, Clone, Default)]
pub enum IdMapLoadingState {
    Loaded,
    #[default]
    Loading,
}

pub struct LoadIdMapPlugin;

impl Plugin for LoadIdMapPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<IdMapLoadingState>()
            .init_asset::<IdMap>()
            .init_resource::<IdMapHandle>()
            .init_asset_loader::<IdMapLoader>()
            .add_systems(OnEnter(GameState::LoadGame), load_id_map)
            .add_systems(
                Update,
                consolidate.run_if(in_state(IdMapLoadingState::Loading)),
            );
    }
}

fn load_id_map(
    mut handle: ResMut<IdMapHandle>,
    path: Res<FetchGamePath>,
    asset_server: Res<AssetServer>,
) {
    let asset = asset_server.load::<IdMap>(&path.id_texture);
    handle.0 = asset;
}

fn consolidate(
    handle: Res<IdMapHandle>,
    assets: Res<Assets<IdMap>>,
    mut state: ResMut<NextState<IdMapLoadingState>>,
) {
    if let Some(x) = assets.get(handle.0.id()) {
        println!("\n\nMap: {:#?}", x.adjacency);
        println!("\n\nMap: {:#?}", x.height);
        println!("\n\nMap: {:#?}", x.width);
        state.set(IdMapLoadingState::Loaded);
    }
}

#[derive(Debug, Error)]
pub enum IdMapLoaderError {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Json(#[from] serde_json::Error),

    #[error(transparent)]
    Image(#[from] image::ImageError),
}

#[derive(Default)]
pub struct IdMapLoader;

impl AssetLoader for IdMapLoader {
    type Asset = IdMap;
    type Settings = ();
    type Error = IdMapLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;

        let img = image::load_from_memory(&bytes)?;
        let rgba = img.to_rgba8();

        let width = rgba.width();
        let height = rgba.height();
        let mut max_id: usize = 0;

        let pixels: Vec<u32> = rgba
            .pixels()
            .map(|px| {
                let g = px[1] as u32;
                let b = px[2] as u32;
                let id = (g << 8) | b;
                max_id = max_id.max(id as usize);
                id
            })
            .collect();

        let mut adjacency = vec![vec![]; max_id + 1];

        find_adjacent_provinces(&mut adjacency, &pixels, width, height);

        return Ok(IdMap {
            width,
            height,
            map: pixels,
            adjacency,
        });
    }
}

fn find_adjacent_provinces(
    adjacency: &mut Vec<Vec<u32>>,
    pixels: &Vec<u32>,
    width: u32,
    height: u32,
) {
    for y in 0..(height - 1) {
        for x in 0..(width - 1) {
            let current = pixels[(y * width + x) as usize];
            let right = pixels[(y * width + x + 1) as usize];
            let down = pixels[((y + 1) * width + x) as usize];

            if current != right {
                add_adjacent(adjacency, current, right);
            }

            if current != down {
                add_adjacent(adjacency, current, down);
            }
        }
    }
}

fn add_adjacent(adjacency: &mut Vec<Vec<u32>>, id1: u32, id2: u32) {
    if !adjacency[id1 as usize].contains(&id2) {
        adjacency[id1 as usize].push(id2);
    }

    if !adjacency[id2 as usize].contains(&id1) {
        adjacency[id2 as usize].push(id1);
    }
}
