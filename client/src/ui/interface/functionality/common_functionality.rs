use super::super::common::*;
use crate::ui::{
    GPUMaterial, GPUMaterialHandle, NO_SELECTED_ID,
    interface::functionality::left_panel_functionality::{ActiveProvince, Refresch},
};
use bevy::prelude::*;
use shared::resources::GameWorld;
pub struct CommonFunctionalityPlugin;

impl Plugin for CommonFunctionalityPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<InterfaceState>()
            .add_systems(
                Update,
                interface_button_colors.run_if(in_state(InterfaceState::Visibile)),
            )
            .add_systems(Update, (change_ui_visibility, select_province))
            .add_systems(OnEnter(InterfaceState::Hidden), hide_interface_root)
            .add_systems(OnEnter(InterfaceState::Visibile), show_interface_root);
    }
}

pub fn interface_button_colors(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut background_color) in &mut interaction_query {
        *background_color = match interaction {
            Interaction::Pressed => PRESSED_BUTTON.into(),
            Interaction::Hovered => HOVERED_BUTTON.into(),
            Interaction::None => NORMAL_BUTTON.into(),
        }
    }
}
pub fn change_ui_visibility(
    keyboard: Res<ButtonInput<KeyCode>>,
    current: Res<State<InterfaceState>>,
    mut next: ResMut<NextState<InterfaceState>>,
) {
    if !keyboard.just_pressed(KeyCode::KeyI) {
        return;
    }

    next.set(match current.get() {
        InterfaceState::Hidden => InterfaceState::Visibile,
        InterfaceState::Visibile => InterfaceState::Hidden,
    });
}

fn hide_interface_root(mut q: Query<&mut Visibility, With<InterfaceRoot>>) {
    for mut v in &mut q {
        *v = Visibility::Hidden;
    }
}

fn show_interface_root(mut q: Query<&mut Visibility, With<InterfaceRoot>>) {
    for mut v in &mut q {
        *v = Visibility::Visible;
    }
}

fn select_province(
    ui_interactions: Query<&Interaction, With<Button>>,
    camera_query: Single<(&Camera, &GlobalTransform)>,
    window: Single<&Window>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    map: Res<GameWorld>,
    mut materials: ResMut<Assets<GPUMaterial>>,
    handle: Res<GPUMaterialHandle>,
    mut province: ResMut<ActiveProvince>,
    mut writer: MessageWriter<Refresch>,
) {
    let pointer_on_ui = ui_interactions.iter().any(|i| *i != Interaction::None);
    if pointer_on_ui {
        return;
    }

    let (camera, camera_transform) = *camera_query;

    if mouse_input.just_pressed(MouseButton::Left) {
        if let Some(cursor_position) = window.cursor_position() {
            if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_position) {
                if let Some(material) = materials.get_mut(handle.0.id()) {
                    if let Some(province_id) = map.get_id(world_pos.x, world_pos.y) {
                        province.0 = province_id;
                        material.selected_id = province_id;
                        writer.write(Refresch {});
                    }
                }
            }
        }
    }
}
