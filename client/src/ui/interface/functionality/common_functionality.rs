use super::super::common::*;
use crate::net::types::ClientOutbox;
use crate::ui::POLITICAL_DRAW;
use crate::ui::{
    ATACK_DRAW, GEOGRAPHICAL_DRAW, GPUMaterial, GPUMaterialHandle, NO_SELECTED_ID,
    interface::{
        desgin::right_panel::MessageLog, functionality::left_panel_functionality::Refresch,
    },
};
use bevy::prelude::*;
use shared::commands_server::CommandServer;
use shared::commands_server::basic::Attack;
use shared::resources::GameWorld;
pub struct CommonFunctionalityPlugin;

impl Plugin for CommonFunctionalityPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<InterfaceState>()
            .add_systems(
                Update,
                interface_button_colors.run_if(in_state(InterfaceState::Visibile)),
            )
            .add_systems(
                Update,
                (
                    change_ui_visibility,
                    select_province.run_if(in_state(AttackState::NoAtack)),
                    timer,
                ),
            )
            .add_systems(OnEnter(InterfaceState::Hidden), hide_interface_root)
            .add_systems(OnEnter(InterfaceState::Visibile), show_interface_root)
            .add_systems(OnEnter(AttackState::Choose), choose_atack_startup)
            .add_systems(Update, choose_atack.run_if(in_state(AttackState::Choose)))
            .add_systems(OnExit(AttackState::Choose), cleanup_attack);
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

fn timer(
    timer: Res<Time>,
    mut materials: ResMut<Assets<GPUMaterial>>,
    handle: Res<GPUMaterialHandle>,
) {
    if let Some(material) = materials.get_mut(handle.0.id()) {
        material.timer = timer.elapsed_secs();
    };
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

fn choose_atack_startup(
    mut materials: ResMut<Assets<GPUMaterial>>,
    handle: Res<GPUMaterialHandle>,
) {
    if let Some(material) = materials.get_mut(handle.0.id()) {
        material.draw_type = ATACK_DRAW;
    }
}

fn choose_atack(
    ui_interactions: Query<&Interaction, With<Button>>,
    camera_query: Single<(&Camera, &GlobalTransform)>,
    window: Single<&Window>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    map: Res<GameWorld>,
    province: Res<ActiveProvince>,
    mut state: ResMut<NextState<AttackState>>,
    outbox: Res<ClientOutbox>,
) {
    let pointer_on_ui = ui_interactions.iter().any(|i| *i != Interaction::None);
    if pointer_on_ui {
        return;
    }

    let (camera, camera_transform) = *camera_query;

    if mouse_input.just_pressed(MouseButton::Left) {
        if let Some(cursor_position) = window.cursor_position() {
            if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_position) {
                if let Some(province_id) = map.get_id(world_pos.x, world_pos.y) {
                    let y = province_id;
                    let x = province.0;
                    if let Ok(s) = CommandServer::Attack(Attack {
                        from_province: x,
                        to_province: y,
                    })
                    .serialize()
                    {
                        if let Err(_) = outbox.0.send(s.clone()) {
                            error!("Could serialize {:?}", s);
                        }
                    }
                    state.set(AttackState::NoAtack);
                }
            }
        }
    }
}

fn cleanup_attack(
    mut materials: ResMut<Assets<GPUMaterial>>,
    handle: Res<GPUMaterialHandle>,
    q: Query<&mut Text, With<MessageLog>>,
) {
    if let Some(material) = &mut materials.get_mut(handle.0.id()) {
        material.draw_type = POLITICAL_DRAW;
    }

    for mut t in q {
        t.0 = "Attack something".to_string();
    }
}
