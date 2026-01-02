use bevy::prelude::*;
use image::Rgba;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Province {
    neighbours: Vec<Rgba<u8>>,
    pixels: Vec<u32>,
}

impl Province {
    pub fn add_neighbour(&mut self, n: Rgba<u8>) {
        if !self.neighbours.contains(&n) {
            self.neighbours.push(n);
        }
    }

    pub fn add_pixel(&mut self, p: u32) {
        self.pixels.push(p);
    }
}
// pixels written
// 1 2 3 4 5
// 6 7 8 9 ...
#[derive(Resource, Default, Debug)]
pub struct Map {
    provinces: HashMap<Rgba<u8>, Province>,
    width: u32,
    height: u32,
    pixels: Vec<Rgba<u8>>,
}

impl Map {
    pub fn set_provinces_map(&mut self, hm: HashMap<Rgba<u8>, Province>) {
        self.provinces = hm;
    }

    pub fn set_map(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    pub fn add_pixel(&mut self, pixel: Rgba<u8>) {
        self.pixels.push(pixel);
    }

    pub fn get_color(&self, x: f32, y: f32) -> Option<&Rgba<u8>> {
        let ix = (x + self.width as f32 / 2.0).round() as i32;
        let iy = (self.height as f32 / 2.0 - y).round() as i32;

        if ix < 0 || iy < 0 || ix >= self.width as i32 || iy >= self.height as i32 {
            return None;
        }

        let idx = (iy as u32 * self.width + ix as u32) as usize;
        self.pixels.get(idx)
    }
}
