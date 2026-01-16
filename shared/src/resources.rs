use bevy::platform::collections::HashSet;
use bevy::prelude::*;
use serde;
use serde::*;

pub const TERRAIN_FLAT: u32 = 0;
pub const TERRAIN_WATER: u32 = 1;
pub const TERRAIN_MOUNTAIN: u32 = 2;
pub const TERRAIN_FOREST: u32 = 3;

pub const NO_OWNER: u32 = 213767;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Province {
    pub owner_id: u32,
    pub terrain_type: u32,
    pub gold_per_second: u32,
    pub level: u32,
    pub army: u32,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Country {
    pub color: Vec4,
    pub flag_path: String,
    pub army: u32,
    pub gold: u32,

    pub is_taken: bool,
    pub war: HashSet<u32>,
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

    pub fn get_province(&self, index: u32) -> Option<&Province> {
        self.provinces.get(index as usize)
    }

    pub fn get_country(&self, province_index: u32) -> Option<&Country> {
        if let Some(p) = self.provinces.get(province_index as usize) {
            return self.countries.get(p.owner_id as usize);
        }
        None
    }
}
