use super::loading_error;
use crate::data::resources::{FetchGamePath, GameState, textures::*};
use anyhow::Result;
use bevy::prelude::*;

#[derive(States, Debug, Hash, Eq, PartialEq, Clone, Default)]
pub enum TexturesLoadingState {
    Loaded,
    #[default]
    Loading,
}

pub struct LoadTexturesPlugin;

impl Plugin for LoadTexturesPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<TexturesLoadingState>()
            .init_asset::<Textures>()
            .init_resource::<TexturesHandle>()
            .add_systems(
                OnEnter(GameState::LoadGame),
                load_texture.pipe(loading_error),
            )
            .add_systems(
                Update,
                consolidate.run_if(in_state(TexturesLoadingState::Loading)),
            );
    }
}

fn load_texture(
    mut assets: ResMut<Assets<Textures>>,
    mut handle: ResMut<TexturesHandle>,
    path: Res<FetchGamePath>,
    asset_server: Res<AssetServer>,
) -> Result<()> {
    let asset = Textures::from(&path.province_texture, &*asset_server)?;
    handle.0 = assets.add(asset);
    Ok(())
}

fn consolidate(
    assets: Res<Assets<Textures>>,
    handle: Res<TexturesHandle>,
    images: Res<Assets<Image>>,
    mut state: ResMut<NextState<TexturesLoadingState>>,
) {
    if let Some(x) = assets.get(handle.0.id()) {
        println!("{:#?}", x);
        if x.is_loaded(images) {
            state.set(TexturesLoadingState::Loaded);
        }
    }
}
