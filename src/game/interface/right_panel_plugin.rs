use bevy::prelude::*;

use crate::data::GameState;

use super::common::InterfaceUiSet;
use super::interface_systems;

pub struct InterfaceRightPanelPlugin;

impl Plugin for InterfaceRightPanelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                interface_systems::ui_interactions::emit_set_political_from_button,
                interface_systems::ui_interactions::emit_set_geographical_from_button,
            )
                .in_set(InterfaceUiSet::Input)
                .run_if(in_state(GameState::Game)),
        )
        .add_systems(
            Update,
            (interface_systems::commands::apply_map_draw_mode_event,)
                .in_set(InterfaceUiSet::Apply)
                .run_if(in_state(GameState::Game)),
        );
    }
}
