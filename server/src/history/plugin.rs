use bevy::{app::PluginGroupBuilder, prelude::*};

use crate::history::{history::HistoryFunctionality, history_graphics::HistoryGraphics};

pub struct HistoryPlugin;

impl PluginGroup for HistoryPlugin {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(HistoryGraphics)
            .add(HistoryFunctionality)
    }
}
