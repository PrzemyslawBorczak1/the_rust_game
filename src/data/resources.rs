use bevy::{prelude::*, render::render_resource::ShaderType};
use serde::{Deserialize, Serialize};

use crate::data::GPUMaterial;

#[derive(States, Debug, Hash, Eq, PartialEq, Clone, Default)]
pub enum GameState {
    #[default]
    Menu,
    LoadGame,
    Game,
}

pub mod terrain_type {
    pub const Flat: u32 = 1;
    pub const Forest: u32 = 2;
    pub const Mountain: u32 = 3;
    pub const Water: u32 = 4;
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, ShaderType)]
pub struct Province {
    owner_id: u32,
    terrain_type: u32,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, ShaderType)]
pub struct Country {
    id: u32,
    color: Vec4,
}

#[derive(Debug, Clone, Asset, TypePath, Default)]
pub struct IdMap {
    pub width: u32,
    pub height: u32,
    pub map: Vec<u32>,
    pub adjacency: Vec<Vec<u32>>,
}

#[derive(Debug, Clone, TypePath, Default, Resource)]
pub struct Textures {
    pub province: Handle<Image>,
}

#[derive(Resource, Default, Debug)]
pub struct GameWorld {
    pub provinces: Vec<Province>,
    pub countries: Vec<Country>,

    pub id: IdMap,
    pub gpu: Handle<GPUMaterial>,
}

impl GameWorld {
    pub fn width(&self) -> u32 {
        self.id.width
    }

    pub fn height(&self) -> u32 {
        self.id.height
    }

    pub fn get_id(&self, x: f32, y: f32) -> Option<u32> {
        let ix = (x + self.width() as f32 / 2.0).round() as i32;
        let iy = (self.height() as f32 / 2.0 - y).round() as i32;

        if ix < 0 || iy < 0 || ix >= self.width() as i32 || iy >= self.height() as i32 {
            return None;
        }

        let idx = (iy as u32 * self.width() + ix as u32) as usize;

        self.id.map.get(idx).map(|o| o.clone())
    }

    pub fn select_province(&self, x: f32, y: f32, materials: &mut Assets<GPUMaterial>) {
        let gpu = match materials.get_mut(self.gpu.id()) {
            None => {
                error!("No GPU material");
                return;
            }
            Some(x) => x,
        };

        if let Some(id) = self.get_id(x, y) {
            gpu.selected_id = id;
        }
    }
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
