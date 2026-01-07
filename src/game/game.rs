use bevy::{app::PluginGroupBuilder, prelude::*};

use super::create_scene::CreateScenePlugin;
pub struct GamePlugin;

impl PluginGroup for GamePlugin {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(CreateScenePlugin)
    }
}
