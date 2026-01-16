use bevy::prelude::*;

use server::data::GameState;

use server::game::GamePlugin;
use server::history::plugin::HistoryPlugin;
use server::menu::MenuPlugin;
use server::startup_game::StartUpGamePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            file_path: "../assets".to_string(),
            ..default()
        }))
        .init_state::<GameState>()
        .add_plugins(MenuPlugin)
        .add_plugins(StartUpGamePlugin)
        .add_plugins(GamePlugin)
        .add_plugins(HistoryPlugin)
        .run();
}
