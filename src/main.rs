use bevy::prelude::*;

mod map_data;
mod map_display;
mod resources;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(map_data::MapDataPlugin)
        .add_plugins(map_display::MapDisplayPlugin)
        .run();
}
