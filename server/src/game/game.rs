use bevy::{app::PluginGroupBuilder, prelude::*};

use super::systems::SystemsPlugin;
use crate::game::net::NetPlugin;

pub struct GamePlugin;

impl PluginGroup for GamePlugin {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(NetPlugin)
            .add(SystemsPlugin)
    }
}
