use bevy::{app::PluginGroupBuilder, prelude::*};

use crate::game::{camera_movement::CameraMovementPlugin, mouse_click::MouseClickPlugin};

use super::create_scene::CreateScenePlugin;
pub struct GamePlugin;

impl PluginGroup for GamePlugin {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(CreateScenePlugin)
            .add(CameraMovementPlugin)
            .add(MouseClickPlugin)
    }
}
