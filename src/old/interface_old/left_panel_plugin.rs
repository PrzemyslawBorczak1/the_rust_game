use bevy::prelude::*;

use crate::data::GameState;

use super::common::{InterfaceUiSet, LeftPanelView};
use super::interface_systems;

pub struct InterfaceLeftPanelPlugin;

impl Plugin for InterfaceLeftPanelPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<LeftPanelView>()
            .add_systems(
                Update,
                (
                    interface_systems::ui_interactions::emit_show_province_from_button,
                    interface_systems::ui_interactions::emit_show_country_from_button,
                )
                    .in_set(InterfaceUiSet::Input)
                    .run_if(in_state(GameState::Game)),
            )
            .add_systems(
                Update,
                (interface_systems::commands::apply_left_panel_view_event,)
                    .in_set(InterfaceUiSet::Apply)
                    .run_if(in_state(GameState::Game)),
            )
            .add_systems(
                Update,
                (interface_systems::left_panel::interface_update_left_panel,)
                    .in_set(InterfaceUiSet::Update)
                    .run_if(in_state(GameState::Game)),
            );
    }
}
