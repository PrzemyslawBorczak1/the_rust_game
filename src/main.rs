use bevy::prelude::*;

use the_rust_game::data::GameState;

use the_rust_game::game::GamePlugin;
use the_rust_game::load_game::LoadGamePlugin;
use the_rust_game::menu::MenuPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .add_plugins(MenuPlugin)
        .add_plugins(LoadGamePlugin)
         .add_plugins(GamePlugin)
        .run();
}
