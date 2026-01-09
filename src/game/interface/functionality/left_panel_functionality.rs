use crate::game::interface::common::InterfaceState;

use super::super::desgin::left_panel::*;
use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum LeftPanelView {
    #[default]
    Province,
    Country,
}

pub struct LeftPanelFunctionalityPlugin;

impl Plugin for LeftPanelFunctionalityPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<LeftPanelView>()
            .add_systems(
                Update,
                (on_country_button_click, on_province_button_click)
                    .run_if(in_state(InterfaceState::Visibile)),
            )
            .add_systems(OnEnter(LeftPanelView::Country), set_country_view)
            .add_systems(OnEnter(LeftPanelView::Province), set_province_view);
    }
}

fn on_country_button_click(
    q: Query<&Interaction, (Changed<Interaction>, With<Button>, With<ShowCountryButton>)>,
    mut next: ResMut<NextState<LeftPanelView>>,
) {
    for interaction in q {
        if *interaction == Interaction::Pressed {
            next.set(LeftPanelView::Country);
        }
    }
}

fn on_province_button_click(
    q: Query<&Interaction, (Changed<Interaction>, With<Button>, With<ShowProvinceButton>)>,
    mut next: ResMut<NextState<LeftPanelView>>,
) {
    for interaction in q {
        if *interaction == Interaction::Pressed {
            next.set(LeftPanelView::Province);
        }
    }
}

fn set_country_view(mut commands: Commands, q_root: Query<Entity, With<LeftPanelBody>>) {
    if let Ok(root) = q_root.single() {
        commands.entity(root).despawn_children();

        commands.entity(root).with_children(|parent| {
            parent.spawn(country_data());
        });
    }
}

fn set_province_view(mut commands: Commands, q_root: Query<Entity, With<LeftPanelBody>>) {
    if let Ok(root) = q_root.single() {
        commands.entity(root).despawn_children();

        commands.entity(root).with_children(|parent| {
            parent.spawn(province_data());
        });
    }
}
