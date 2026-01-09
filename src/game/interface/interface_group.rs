use bevy::{app::PluginGroupBuilder, prelude::*};

use super::{desgin, functionality};

pub struct InterfacePlugin;

impl PluginGroup for InterfacePlugin {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(desgin::DesignPlugin)
            .add_group(functionality::FunctionalityPlugin)
    }
}
