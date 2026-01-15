use super::loading_error;
use super::resources::*;
use crate::data::{FetchGamePath, GameState};
use anyhow::{Context, Result};
use bevy::prelude::*;

#[derive(States, Debug, Hash, Eq, PartialEq, Clone, Default)]
pub enum CountryLoadingState {
    Loaded,
    #[default]
    Loading,
}

pub struct LoadVecCountryPlugin;

impl Plugin for LoadVecCountryPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<CountryLoadingState>()
            .init_asset::<VecCountry>()
            .init_resource::<VecCountryHandle>()
            .add_systems(
                OnEnter(GameState::LoadGame),
                load_country.pipe(loading_error),
            )
            .add_systems(
                Update,
                consolidate_country.run_if(in_state(CountryLoadingState::Loading)),
            );
    }
}

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

fn consolidate_country(
    assets: Res<Assets<VecCountry>>,
    handle: Res<VecCountryHandle>,
    mut state: ResMut<NextState<CountryLoadingState>>,
) {
    if let Some(x) = assets.get(handle.0.id()) {
        println!("{x:#?}");
        state.set(CountryLoadingState::Loaded);
    }
}
