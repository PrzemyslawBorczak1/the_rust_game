use crate::data::loaders::*;
use crate::data::{FetchGamePath, FetchHandles, GameState, VecCountry, VecProvince};
use bevy::prelude::*;

pub struct LoadAssetsPlugin;
impl Plugin for LoadAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<VecProvince>()
            .init_asset::<VecCountry>()
            .init_resource::<FetchHandles>()
            .init_asset_loader::<VecProvinceLoader>()
            .init_asset_loader::<VecCountryLoader>()
            .add_systems(OnEnter(GameState::LoadGame), load)
            .add_systems(Update, look);
    }
}

fn load(
    asset_server: Res<AssetServer>,
    mut handles: ResMut<FetchHandles>,
    data: Res<FetchGamePath>,
) {
    handles.id_texture = asset_server.load(data.id_texture.clone());
    handles.province_texture = asset_server.load(data.province_texture.clone());

    handles.vec_country = asset_server.load(data.vec_country.clone());
    handles.vec_provinces = asset_server.load(data.vec_provinces.clone());
}

// todo delete
fn look(
    vec_prov: Res<Assets<VecProvince>>,
    vec_country: Res<Assets<VecCountry>>,
    handles: Res<FetchHandles>,
) {
    if let Some(map) = vec_prov.get(&handles.vec_provinces) {
        println!("{:?}", map.0);
    } else {
        println!("prov still loading");
    }

    if let Some(map) = vec_country.get(&handles.vec_country) {
        println!("{:?}", map.0);
    } else {
        println!("country still loading");
    }
}
