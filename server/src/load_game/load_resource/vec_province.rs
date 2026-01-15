use super::loading_error;
use crate::data::{FetchGamePath, GameState};
use anyhow::{Context, Result};
use bevy::prelude::*;

use super::resources::*;

#[derive(States, Debug, Hash, Eq, PartialEq, Clone, Default)]
pub enum ProvinceLoadingState {
    Loaded,
    #[default]
    Loading,
}

pub struct LoadVecProvincePlugin;

impl Plugin for LoadVecProvincePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<ProvinceLoadingState>()
            .init_asset::<VecProvince>()
            .init_resource::<VecProvinceHandle>()
            .add_systems(
                OnEnter(GameState::LoadGame),
                load_province.pipe(loading_error),
            )
            .add_systems(
                Update,
                consolidate_province.run_if(in_state(ProvinceLoadingState::Loading)),
            );
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

fn consolidate_province(
    assets: Res<Assets<VecProvince>>,
    handle: Res<VecProvinceHandle>,
    mut state: ResMut<NextState<ProvinceLoadingState>>,
) {
    if let Some(x) = assets.get(handle.0.id()) {
        println!("{x:#?}");
        state.set(ProvinceLoadingState::Loaded);
    }
}
