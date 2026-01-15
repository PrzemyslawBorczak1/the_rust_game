use bevy::prelude::*;

use server::data::GameState;

use server::game::GamePlugin;
use server::menu::MenuPlugin;
use server::startup_game::StartUpGamePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            // path is relative to the "project root" Bevy resolves (manifest dir / cwd rules)
            // so in a workspace, you can just go up to workspace root:
            file_path: "../assets".to_string(),
            ..default()
        }))
        .init_state::<GameState>()
        .add_plugins(MenuPlugin)
        .add_plugins(StartUpGamePlugin)
        .add_plugins(GamePlugin)
        .run();
}
