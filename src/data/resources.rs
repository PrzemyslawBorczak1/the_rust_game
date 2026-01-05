use bevy::{prelude::*, render::render_resource::TextureFormat};
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Country {
    id: u32,
}

#[derive(Resource, Default, Debug)]
pub struct Map {
    province_id: Vec<Province>,
    countries: Vec<Country>,

    pub width: u32,
    pub height: u32,
    pixels: Vec<u8>,
    ready: bool,
}
// todo error
impl Map {
    pub fn set_map_from_image(&mut self, images: &Assets<Image>, handle: &Handle<Image>) {
        if let Some(image) = images.get(handle) {
            self.width = image.texture_descriptor.size.width;
            self.height = image.texture_descriptor.size.height;

            let format = Some(image.texture_descriptor.format);

            if Some(TextureFormat::Rgba8Unorm) != format {
                eprintln!(
                    "Warning: Expected Rgba8Unorm, got {:?}",
                    image.texture_descriptor.format
                );
            }

            if let Some(data) = &image.data {
                self.pixels = data.clone();
            }
            self.ready = true;
        }

        println!("{:?}", self.width);
        println!("{:?}", self.height);
    }
    pub fn get_color(&self, x: f32, y: f32) -> Option<[u8; 4]> {
        let ix = (x + self.width as f32 / 2.0).round() as i32;
        let iy = (self.height as f32 / 2.0 - y).round() as i32;

        if ix < 0 || iy < 0 || ix >= self.width as i32 || iy >= self.height as i32 {
            return None;
        }

        let idx = (iy as u32 * self.width + ix as u32) as usize * 4;

        self.pixels
            .get(idx..idx + 4)
            .and_then(|b| b.try_into().ok())
    }

    pub fn is_ready(&self) -> bool {
        self.ready
    }
}

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

#[derive(Debug, Clone, Asset, TypePath)]
pub struct VecProvince(pub Vec<Province>);

#[derive(Debug, Clone, Asset, TypePath)]
pub struct VecCountry(pub Vec<Country>);

#[derive(Debug, Resource, Default)]
pub struct FetchHandles {
    pub id_texture: Handle<Image>,
    pub province_texture: Handle<Image>,

    pub vec_provinces: Handle<VecProvince>,
    pub vec_country: Handle<VecCountry>,
}

// impl GameData {
//     pub fn is_loaded(&self, asset_server: Res<AssetServer>) -> bool {
//         if asset_server.load_state(&self.id_path).is_loaded()
//             && asset_server.load_state(&self.texture_path).is_loaded()
//         {
//             return true;
//         }
//         false
//     }

//     pub fn new(id_path: &String, texture_path: &String, asset_server: Res<AssetServer>) -> Self {
//         Self {
//             id_path: asset_server.load(AssetPath::from_path(Path::new(id_path))),
//             texture_path: asset_server.load(AssetPath::from_path(Path::new(texture_path))),
//         }
//     }
// }
