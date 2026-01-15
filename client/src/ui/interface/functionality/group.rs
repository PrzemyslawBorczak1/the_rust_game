use super::*;
use bevy::{app::PluginGroupBuilder, prelude::*};

pub struct FunctionalityPlugin;

impl PluginGroup for FunctionalityPlugin {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(common_functionality::CommonFunctionalityPlugin)
            .add(left_panel_functionality::LeftPanelFunctionalityPlugin)
            .add(right_panel_functionality::RightPanelFunctionalityPlugin)
    }
}
