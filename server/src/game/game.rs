use bevy::{app::PluginGroupBuilder, prelude::*};

use super::systems::SystemsPlugin;
use crate::game::{
    ai::AiPlugin, graphics::GameGraphicsPlugin, history::HistoryPlugin, net::NetPlugin,
};

pub struct GamePlugin;

impl PluginGroup for GamePlugin {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(NetPlugin)
            .add(SystemsPlugin)
            .add(GameGraphicsPlugin)
            .add(AiPlugin)
            .add(HistoryPlugin)
    }
}
