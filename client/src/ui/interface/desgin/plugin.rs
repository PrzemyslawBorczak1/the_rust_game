use bevy::prelude::*;
use bevy::ui::FocusPolicy;

use super::super::common::*;
use super::left_panel::left_panel;
use super::right_panel::right_panel;

pub struct DesignPlugin;
impl Plugin for DesignPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, interface_stratup);
    }
}

fn interface_stratup(mut commands: Commands, mut visibility: ResMut<NextState<InterfaceState>>) {
    visibility.set(InterfaceState::Visibile);

    commands.spawn((
        InterfaceRoot,
        Visibility::Visible,
        FocusPolicy::Block,
        root_node(),
        children![left_panel(), right_panel()],
    ));
}

fn root_node() -> Node {
    Node {
        width: percent(100),
        height: percent(100),
        justify_content: JustifyContent::SpaceBetween,
        align_items: AlignItems::Stretch,
        ..default()
    }
}
