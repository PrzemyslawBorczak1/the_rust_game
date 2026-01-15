use bevy::{prelude::*, render::render_resource::ShaderType};
use serde;
use serde::*;

pub mod terrain_type {
    pub const Flat: u32 = 1;
    pub const Forest: u32 = 2;
    pub const Mountain: u32 = 3;
    pub const Water: u32 = 4;
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, ShaderType)]
pub struct Province {
    pub owner_id: u32,
    pub terrain_type: u32,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, ShaderType)]
pub struct Country {
    pub id: u32,
    pub color: Vec4,
}

#[derive(Debug, Clone, Asset, TypePath, Default, Serialize, Deserialize)]
pub struct IdMap {
    pub width: u32,
    pub height: u32,
    pub map: Vec<u32>,
    pub adjacency: Vec<Vec<u32>>,
}

#[derive(Resource, Default, Debug, Serialize, Deserialize)]
pub struct GameWorld {
    pub provinces: Vec<Province>,
    pub countries: Vec<Country>,

    pub id: IdMap,
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

    pub fn clone(&self) -> Self {
        Self {
            provinces: self.provinces.clone(),
            countries: self.countries.clone(),

            id: self.id.clone(),
        }
    }
}
