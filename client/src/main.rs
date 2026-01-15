use bevy::prelude::*;

use client::net::ConnectionPlugin;
use client::startup_plugin::StartupPlugin;
use client::ui::UIGroup;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            file_path: "../assets".to_string(),
            ..default()
        }))
        .add_plugins(StartupPlugin)
        .add_plugins(ConnectionPlugin)
        .add_plugins(UIGroup)
        .run();
}
