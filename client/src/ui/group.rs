use super::{
    camera_movement::CameraMovementPlugin,  startup::StartupPlugin,
};

use super::interface::InterfaceGroup;
use bevy::{app::PluginGroupBuilder, prelude::*};

pub struct UIGroup;

impl PluginGroup for UIGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add_group(InterfaceGroup)
            .add(CameraMovementPlugin)
            .add(StartupPlugin)
    }
}
