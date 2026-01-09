use bevy::prelude::*;

use crate::data::GameState;

use super::common::InterfaceUiSet;
use super::interface_systems;

pub struct InterfaceControlsPlugin;

impl Plugin for InterfaceControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                interface_systems::ui_interactions::interface_button_colors,
                interface_systems::ui_interactions::emit_toggle_ui_from_button,
                interface_systems::ui_interactions::interface_toggle_keyboard,
            )
                .in_set(InterfaceUiSet::Input)
                .run_if(in_state(GameState::Game)),
        )
        .add_systems(
            Update,
            (interface_systems::commands::apply_toggle_ui_event,)
                .in_set(InterfaceUiSet::Apply)
                .run_if(in_state(GameState::Game)),
        )
        .add_systems(
            Update,
            (interface_systems::responsive::interface_apply_responsive_layout,)
                .in_set(InterfaceUiSet::Update)
                .run_if(in_state(GameState::Game)),
        );
    }
}
