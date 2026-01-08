use super::load_resource::*;
use crate::data::{GameWorld, resources::*};
use bevy::prelude::*;

#[derive(States, Debug, Hash, Eq, PartialEq, Clone, Default)]
pub enum LoadingState {
    #[default]
    Loading,
    Finished,
}

pub struct FinishLoadingPlugin;

impl Plugin for FinishLoadingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameWorld>()
            .init_state::<LoadingState>()
            .add_systems(
                Update,
                finish.run_if(in_state(GameState::LoadGame).and(in_state(LoadingState::Loading))),
            );
    }
}

// add textures asset
fn finish(
    prov: Res<Assets<VecProvince>>,
    prov_handle: Res<VecProvinceHandle>,

    countries: Res<Assets<VecCountry>>,
    country_handle: Res<VecCountryHandle>,

    map: Res<Assets<IdMap>>,
    map_handle: Res<IdMapHandle>,

    textures: Res<Assets<TexturesAsset>>,
    textures_handle: Res<TexturesHandle>,

    mut world: ResMut<GameWorld>,
    mut loading_state: ResMut<NextState<LoadingState>>,
    mut commands: Commands,
) {
    if let Some(prov) = prov.get(prov_handle.0.id()) {
        if let Some(country) = countries.get(country_handle.0.id()) {
            if let Some(map) = map.get(map_handle.0.id()) {
                if let Some(textures_asset) = textures.get(textures_handle.0.id()) {
                    world.provinces = prov.0.clone();
                    world.countries = country.0.clone();

                    world.id = map.clone();
                    commands.remove_resource::<VecProvinceHandle>();
                    commands.remove_resource::<VecCountryHandle>();
                    commands.remove_resource::<IdMapHandle>();
                    commands.remove_resource::<TexturesHandle>();

                    commands.insert_resource(textures_asset.clone().as_resource());

                    loading_state.set(LoadingState::Finished);
                    println!("\n\nFinished\n\n");
                }
            }
        }
    }
}
