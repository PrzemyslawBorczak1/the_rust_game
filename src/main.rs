use bevy::prelude::*;

mod game;
mod menu;

#[derive(States, Debug, Hash, Eq, PartialEq, Clone, Default)]
enum GameState {
    #[default]
    Menu,
    Game,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .add_plugins(menu::MenuPlugin)
        .add_plugins(game::StartGame)
        .run();
}
