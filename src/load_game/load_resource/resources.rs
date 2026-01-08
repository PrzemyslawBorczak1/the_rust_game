use anyhow::Result;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::Path;

pub use country::*;
pub use id_map::*;
pub use province::*;
pub use textures::*;

use crate::data::*;

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

pub mod textures {
    pub use super::*;

    #[derive(Serialize, Deserialize)]
    struct Paths {
        province: String,
    }
    #[derive(Debug, Asset, Clone, TypePath, Default)]
    pub struct TexturesAsset {
        pub province: Handle<Image>,
    }

    impl TexturesAsset {
        pub fn from(path_in_assets: &String, asset_server: &AssetServer) -> Result<Self> {
            let path = Path::new("assets").join(path_in_assets);
            let bytes = std::fs::read(path)?;
            let paths: Paths = serde_json::from_slice(&bytes)?;
            let mut ret = TexturesAsset::default();

            ret.province = asset_server.load(paths.province);

            Ok(ret)
        }

        pub fn is_loaded(&self, images: Res<Assets<Image>>) -> bool {
            if let Some(_) = images.get(self.province.id()) {
                return true;
            }
            false
        }

        pub fn as_resource(self) -> Textures {
            Textures {
                province: self.province,
            }
        }
    }

    #[derive(Resource, Default)]
    pub struct TexturesHandle(pub Handle<TexturesAsset>);
}
