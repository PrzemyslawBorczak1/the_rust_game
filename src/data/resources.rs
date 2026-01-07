use anyhow::Result;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::Path;

pub use country::*;
pub use id_map::*;
pub use province::*;
pub use textures::*;

pub mod province {
    use super::*;

    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    pub enum TerrainType {
        #[default]
        Flat,
        Forest,
        Mountain,
        Water,
    }

    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    pub struct Province {
        owner_id: u32,
        terrain_type: TerrainType,
    }
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
    use bevy::render::render_resource::ShaderType;

    use super::*;
    #[derive(States, Debug, Hash, Eq, PartialEq, Clone, Default)]
    pub enum CountryLoadingState {
        Loaded,
        #[default]
        Loading,
    }
    #[derive(Debug, Default, Clone, Serialize, Deserialize, ShaderType)]
    pub struct Country {
        id: u32,
    }

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
    #[derive(Debug, Clone, Asset, TypePath, Default)]
    pub struct IdMap {
        pub width: u32,
        pub height: u32,
        pub map: Vec<u32>,
        pub adjacency: Vec<Vec<u32>>,
    }

    #[derive(Resource, Default)]
    pub struct IdMapHandle(pub Handle<IdMap>);
}

pub mod textures {
    pub use super::*;

    #[derive(Debug, Clone, Asset, TypePath, Default)]
    pub struct Textures {
        pub province: Handle<Image>,
    }

    #[derive(Serialize, Deserialize)]
    struct Paths {
        province: String,
    }

    impl Textures {
        pub fn from(path_in_assets: &String, asset_server: &AssetServer) -> Result<Self> {
            let path = Path::new("assets").join(path_in_assets);
            let bytes = std::fs::read(path)?;
            let paths: Paths = serde_json::from_slice(&bytes)?;
            let mut ret = Textures::default();

            ret.province = asset_server.load(paths.province);

            Ok(ret)
        }

        pub fn is_loaded(&self, images: Res<Assets<Image>>) -> bool {
            if let Some(_) = images.get(self.province.id()) {
                return true;
            }
            false
        }
    }

    #[derive(Resource, Default)]
    pub struct TexturesHandle(pub Handle<Textures>);
}

#[derive(Resource, Default, Debug)]
pub struct GameWorld {
    pub provinces: Vec<Province>,
    pub countries: Vec<Country>,

    pub id: IdMap,
}
// todo error
// impl Map {
//     pub fn set_map_from_image(&mut self, images: &Assets<Image>, handle: &Handle<Image>) {
//         if let Some(image) = images.get(handle) {
//             self.width = image.texture_descriptor.size.width;
//             self.height = image.texture_descriptor.size.height;

//             let format = Some(image.texture_descriptor.format);

//             if Some(TextureFormat::Rgba8Unorm) != format {
//                 eprintln!(
//                     "Warning: Expected Rgba8Unorm, got {:?}",
//                     image.texture_descriptor.format
//                 );
//             }

//             if let Some(data) = &image.data {
//                 self.pixels = data.clone();
//             }
//             self.ready = true;
//         }

//         println!("{:?}", self.width);
//         println!("{:?}", self.height);
//     }
//     pub fn get_color(&self, x: f32, y: f32) -> Option<[u8; 4]> {
//         let ix = (x + self.width as f32 / 2.0).round() as i32;
//         let iy = (self.height as f32 / 2.0 - y).round() as i32;

//         if ix < 0 || iy < 0 || ix >= self.width as i32 || iy >= self.height as i32 {
//             return None;
//         }

//         let idx = (iy as u32 * self.width + ix as u32) as usize * 4;

//         self.pixels
//             .get(idx..idx + 4)
//             .and_then(|b| b.try_into().ok())
//     }

//     pub fn is_ready(&self) -> bool {
//         self.ready
//     }
// }

#[derive(States, Debug, Hash, Eq, PartialEq, Clone, Default)]
pub enum GameState {
    #[default]
    Menu,
    LoadGame,
    Game,
}

#[derive(Debug, Default, Resource)]
pub struct FetchGamePath {
    pub id_texture: String,
    pub province_texture: String,
    pub vec_provinces: String,
    pub vec_country: String,
}

#[derive(Debug, Default, Resource)]
pub struct SaveGamePath {
    pub id_texture: String,
    pub province_texture: String,
    pub vec_provinces: String,
    pub vec_country: String,
}

#[derive(Debug, Resource, Default)]
pub struct FetchHandles {
    pub province_texture: Handle<Image>,

    pub id_map: Handle<IdMap>,
    pub vec_provinces: Handle<VecProvince>,
    pub vec_country: Handle<VecCountry>,
}
