use anyhow::Result;
use bevy::prelude::*;
use std::path::Path;

pub use country::*;
pub use id_map::*;
pub use province::*;

use shared::resources::{Province, Country, IdMap};

pub mod province {
    use super::*;

    #[derive(Debug, Clone, Asset, TypePath)]
    pub struct VecProvince(pub Vec<Province>);
    impl VecProvince {
        pub fn deserialize(path_in_assets: &String) -> Result<Self> {
            let path = Path::new("assets").join(path_in_assets);
            let bytes = std::fs::read(path)?;
            let provinces: Vec<Province> = serde_json::from_slice(&bytes)?;
            Ok(Self(provinces))
        }
    }

    #[derive(Resource, Default)]
    pub struct VecProvinceHandle(pub Handle<VecProvince>);
}

pub mod country {
    use super::*;

    #[derive(Debug, Clone, Asset, TypePath)]
    pub struct VecCountry(pub Vec<Country>);
    impl VecCountry {
        pub fn deserialize(path_in_assets: &String) -> Result<Self> {
            let path = Path::new("assets").join(path_in_assets);
            let bytes = std::fs::read(path)?;
            let country: Vec<Country> = serde_json::from_slice(&bytes)?;
            Ok(Self(country))
        }
    }

    #[derive(Resource, Default)]
    pub struct VecCountryHandle(pub Handle<VecCountry>);
}

pub mod id_map {
    use super::*;

    #[derive(Resource, Default)]
    pub struct IdMapHandle(pub Handle<IdMap>);
}
