use bevy::{prelude::*, render::render_phase::NonMeshEntities};
use image::Rgba;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub enum TerrainType {
    #[default]
    Flat,
    Forest,
    Mountain,
    Water,
}

#[derive(Debug, Default)]
pub struct Province {
    neighbours: Vec<[u8; 4]>,
    owner_id: u32,
    terrain_type: TerrainType,
}

#[derive(Debug, Default)]
pub struct Country {
    id: u32,
}

#[derive(Resource, Default, Debug)]
pub struct Map {
    province_id: HashMap<[u8; 4], Province>,
    countries: HashMap<u32, Country>,
    width: u32,
    height: u32,
    pixels: Vec<u8>,
}
// todo error
impl Map {
    pub fn set_map_from_image(&mut self, images: &Assets<Image>, handle: &Handle<Image>) {
        if let Some(img) = images.get(handle) {
            let data = img
                .data
                .as_ref()
                .expect("Image has no CPU data (texture is GPU-only)");

            self.width = img.texture_descriptor.size.width;
            self.height = img.texture_descriptor.size.height;

            self.pixels.clear();
            self.pixels.reserve_exact(data.len());
            self.pixels.extend_from_slice(data);
        }
    }
    pub fn get_color(&self, x: f32, y: f32) -> Option<[u8; 4]> {
        let ix = (x + self.width as f32 / 2.0).round() as i32;
        let iy = (self.height as f32 / 2.0 - y).round() as i32;

        if ix < 0 || iy < 0 || ix >= self.width as i32 || iy >= self.height as i32 {
            return None;
        }

        let idx = (iy as u32 * self.width + ix as u32) as usize;

        self.pixels
            .get(idx..idx + 4)
            .and_then(|b| b.try_into().ok())
    }
}
