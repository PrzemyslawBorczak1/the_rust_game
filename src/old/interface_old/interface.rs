use bevy::prelude::*;

use crate::data::GameState;

use super::common::*;
use super::controls_plugin::InterfaceControlsPlugin;
use super::interface_design;
use super::left_panel_plugin::InterfaceLeftPanelPlugin;
use super::right_panel_plugin::InterfaceRightPanelPlugin;

pub struct InterfacePlugin;

impl Plugin for InterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<ToggleUiEvent>();
        app.add_message::<SetLeftPanelViewEvent>();
        app.add_message::<SetMapDrawModeEvent>();

        app.configure_sets(
            Update,
            (
                InterfaceUiSet::Input,
                InterfaceUiSet::Apply,
                InterfaceUiSet::Update,
            )
                .chain(),
        );

        app.add_plugins((
            InterfaceControlsPlugin,
            InterfaceLeftPanelPlugin,
            InterfaceRightPanelPlugin,
        ));

        app.add_systems(OnEnter(GameState::Game), interface_on_enter);
    }
}

fn interface_on_enter(mut commands: Commands, mut next_view: ResMut<NextState<LeftPanelView>>) {
    next_view.set(LeftPanelView::Province);
    commands.insert_resource(MapDrawMode::Political);
    interface_design::spawn::spawn_interface(&mut commands);
}
