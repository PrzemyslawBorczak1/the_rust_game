use bevy::prelude::{App, Plugin, ResMut, Resource, Startup};
use image::{ImageBuffer, ImageReader, Rgb};
use std::collections::HashMap;

use super::resources::{Map, Province};

pub struct MapDataPlugin;

impl Plugin for MapDataPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MapData::default())
            .insert_resource(Map::default())
            .add_systems(Startup, set_map);
    }
}

#[derive(Resource, Default, Debug)]
pub struct MapData {
     assets_map_path: String,
    pub width: u32,
    pub height: u32,
}

impl MapData{
    pub fn get_path(&self) -> String{
        "assets/".to_string() + self.assets_map_path.as_str()
    }

     pub fn get_path_in_assets(&self) -> String{
        self.assets_map_path.clone()
    }
}

fn set_map(map_res: ResMut<Map>, mut data: ResMut<MapData>) {
    data.assets_map_path = "map3.png".to_string();

    let rgb: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageReader::open(data.get_path())
        .unwrap()
        .decode()
        .unwrap()
        .to_rgb8();

    let (w, h) = rgb.dimensions();

    data.width = w;
    data.height = h;

    provinces_from_image(map_res, rgb);
}

fn provinces_from_image(mut map_res: ResMut<Map>, rgb: ImageBuffer<Rgb<u8>, Vec<u8>>) {
    let mut hm = HashMap::<Rgb<u8>, Province>::new();
    let mut map = Map::default();

    let (w, h) = rgb.dimensions();

    map.set_map(w, h);

    for y in 0..h {
        for x in 0..w {
            let c = *rgb.get_pixel(x, y);
            hm.entry(c).or_default();

            if x + 1 < w {
                let right = *rgb.get_pixel(x + 1, y);
                if right != c {
                    add_border(&mut hm, c, right);
                }
            }

            if y + 1 < h {
                let down = *rgb.get_pixel(x, y + 1);
                if down != c {
                    add_border(&mut hm, c, down);
                }
            }

            if let Some(v) = hm.get_mut(&c) {
                v.add_pixel(y * w + x);
            }

            map.add_pixel(c);
        }
    }

    map.set_provinces_map(hm);

    *map_res = map;
}

fn add_border(hm: &mut HashMap<Rgb<u8>, Province>, a: Rgb<u8>, b: Rgb<u8>) {
    hm.entry(a).or_default();
    hm.entry(b).or_default();

    if let Some(pa) = hm.get_mut(&a) {
        pa.add_neighbour(b);
    }

    if let Some(pb) = hm.get_mut(&b) {
        pb.add_neighbour(a);
    }
}
