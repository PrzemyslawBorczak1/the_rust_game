use bevy::prelude::*;

use super::super::common::*;

pub struct CommonFunctionalityPlugin;

impl Plugin for CommonFunctionalityPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<InterfaceState>()
            .add_systems(
                Update,
                interface_button_colors.run_if(in_state(InterfaceState::Visibile)),
            )
            .add_systems(Update, change_ui_visibility)
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
