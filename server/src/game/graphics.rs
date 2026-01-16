use bevy::prelude::*;

use crate::data::GameState;

pub struct GameGraphicsPlugin;

impl Plugin for GameGraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpeedMultiplier>()
            .add_systems(OnEnter(GameState::Game), startup)
            .add_systems(Update, ui_buttons_system);
    }
}

#[derive(Component)]
struct UiRoot;

#[derive(Component, Copy, Clone)]
enum UiAction {
    Start,
    Speed2x,
}

#[derive(Resource, Default)]
pub struct SpeedMultiplier(pub f32);

// ---- startup ----

pub fn startup(mut commands: Commands) {
    spawn_camera(&mut commands);
    spawn_simple_menu(&mut commands);
}

fn spawn_camera(commands: &mut Commands) {
    commands.spawn(Camera2d::default());
}

fn spawn_simple_menu(commands: &mut Commands) {
    commands.spawn((
        UiRoot,
        Node {
            width: percent(100.0),
            height: percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![spawn_menu_column()],
    ));
}

fn spawn_menu_column() -> impl Bundle {
    (
        Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            row_gap: px(18.0),
            ..default()
        },
        children![
            spawn_button("Start", UiAction::Start),
            spawn_button("Speedup 2x", UiAction::Speed2x)
        ],
    )
}

fn spawn_button(label: &'static str, action: UiAction) -> impl Bundle {
    (
        Button,
        action,
        Node {
            width: px(260.0),
            height: px(64.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            margin: UiRect::all(px(6.0)),
            ..default()
        },
        BackgroundColor(Color::srgb(0.18, 0.18, 0.18)),
        children![(
            Text::new(label),
            TextFont {
                font_size: 34.0,
                ..default()
            },
        )],
    )
}

// ---- interaction ----

pub fn ui_buttons_system(
    mut q: Query<
        (&Interaction, &UiAction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut speed: ResMut<SpeedMultiplier>,
) {
    for (interaction, action, mut bg) in &mut q {
        match *interaction {
            Interaction::Pressed => {
                *bg = BackgroundColor(Color::srgb(0.10, 0.10, 0.10));
                handle_action(*action, &mut speed);
            }
            Interaction::Hovered => {
                *bg = BackgroundColor(Color::srgb(0.24, 0.24, 0.24));
            }
            Interaction::None => {
                *bg = BackgroundColor(Color::srgb(0.18, 0.18, 0.18));
            }
        }
    }
}

fn handle_action(action: UiAction, speed: &mut SpeedMultiplier) {
    match action {
        UiAction::Start => {
            info!("Start pressed");
            // put your "start game" trigger here
        }
        UiAction::Speed2x => {
            speed.0 = 2.0;
            info!("Speed set to 2x");
        }
    }
}
