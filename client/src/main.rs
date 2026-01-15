use bevy::prelude::*;

use client::net::ConnectionPlugin;
use client::startup_plugin::StartupPlugin;
use client::ui::UIGroup;

fn main() {
    App::new()
        .add_plugins(StartupPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugins(ConnectionPlugin)
        .add_plugins(UIGroup)
        .run();
}
