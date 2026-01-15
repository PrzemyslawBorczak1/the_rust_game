use bevy::prelude::*;

use bevy::state::app::StatesPlugin;
use server::data::GameState;

use server::game::GamePlugin;
use server::load_game::LoadGamePlugin;
use server::menu::MenuPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .add_plugins(MenuPlugin)
        .add_plugins(LoadGamePlugin)
        .add_plugins(GamePlugin)
        .run();
}
