use crate::ui::gpu::GPUMaterialHandle;
use crate::ui::{GEOGRAPHICAL_DRAW, POLITICAL_DRAW};

use super::super::super::gpu::GPUMaterial;
use super::super::common::InterfaceState;
use super::super::desgin::right_panel::*;

use bevy::prelude::*;

pub struct RightPanelFunctionalityPlugin;

impl Plugin for RightPanelFunctionalityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (on_political_click, on_geographical_click).run_if(in_state(InterfaceState::Visibile)),
        )
        .add_systems(Update, change_ui_visibility);
    }
}

pub fn on_political_click(
    mut q: Query<&Interaction, (Changed<Interaction>, With<Button>, With<SetPoliticalButton>)>,
    mut materials: ResMut<Assets<GPUMaterial>>,
    handle: Res<GPUMaterialHandle>,
) {
    for interaction in &mut q {
        if *interaction == Interaction::Pressed {
            if let Some(gpu) = materials.get_mut(handle.0.id()) {
                gpu.draw_type = POLITICAL_DRAW;
                println!("{}", gpu.selected_id);
            }
        }
    }
}

pub fn on_geographical_click(
    mut q: Query<
        &Interaction,
        (
            Changed<Interaction>,
            With<Button>,
            With<SetGeographicalButton>,
        ),
    >,
    mut materials: ResMut<Assets<GPUMaterial>>,
    handle: Res<GPUMaterialHandle>,
) {
    for interaction in &mut q {
        if *interaction == Interaction::Pressed {
            if let Some(gpu) = materials.get_mut(handle.0.id()) {
                gpu.draw_type = GEOGRAPHICAL_DRAW;
                println!("{}", gpu.selected_id);
            }
        }
    }
}

pub fn change_ui_visibility(
    mut q: Query<&Interaction, (Changed<Interaction>, With<Button>, With<HideUiButton>)>,
    current: Res<State<InterfaceState>>,
    mut next: ResMut<NextState<InterfaceState>>,
) {
    for interaction in &mut q {
        if *interaction == Interaction::Pressed {
            next.set(match current.get() {
                InterfaceState::Hidden => InterfaceState::Visibile,
                InterfaceState::Visibile => InterfaceState::Hidden,
            });
        }
    }
}
