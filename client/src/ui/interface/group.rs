use bevy::{app::PluginGroupBuilder, prelude::*};

use super::{desgin, functionality};

pub struct InterfaceGroup;

impl PluginGroup for InterfaceGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(desgin::DesignPlugin)
            .add_group(functionality::FunctionalityPlugin)
    }
}
