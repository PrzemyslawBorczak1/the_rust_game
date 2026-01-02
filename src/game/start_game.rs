use bevy::{app::PluginGroupBuilder, prelude::*};

use super::set_map::SetMapPlugin;

pub struct StartGame;

impl PluginGroup for StartGame {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
        .add(SetMapPlugin)
    }
}
