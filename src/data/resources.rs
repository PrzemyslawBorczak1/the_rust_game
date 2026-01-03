use bevy::{
    prelude::*,
    render::{render_phase::NonMeshEntities, render_resource::TextureFormat},
};
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
    StartGame,
    Game,
}
