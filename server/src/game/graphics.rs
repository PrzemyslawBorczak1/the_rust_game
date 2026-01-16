use std::fs;

use bevy::{ecs::world, prelude::*};
use shared::resources::GameWorld;

use crate::{
    data::{GameState, SaveGamePath},
    game::{ai::AiState, systems::GlobalTimer},
    history::history::History,
};

pub struct GameGraphicsPlugin;

impl Plugin for GameGraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpeedMultiplier(2.0))
            .add_systems(OnEnter(GameState::Game), startup)
            .add_systems(
                Update,
                (
                    button_system,
                    on_speed_button_click,
                    on_save_button_click,
                    on_start_button_click,
                    on_history_button_click,
                ),
            );
    }
}

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

#[derive(Resource, Default)]
pub struct SpeedMultiplier(pub f32);

#[derive(Component)]
struct GameUiRoot;

#[derive(Component)]
pub struct StartButton;

#[derive(Component)]
pub struct Speedup2xButton;

#[derive(Component)]
pub struct SaveButton;

#[derive(Component)]
pub struct HistoryButton;

fn startup(mut commands: Commands) {
    println!("Stratup!");

    commands.spawn(Camera2d::default());

    commands.spawn((
        DespawnOnExit(GameState::Game),
        GameUiRoot,
        Node {
            width: percent(100.0),
            height: percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![(
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: px(14.0),
                ..default()
            },
            children![
                (
                    Text::new("Game"),
                    TextFont {
                        font_size: 67.0,
                        ..default()
                    },
                    Node {
                        margin: UiRect::all(px(50.0)),
                        ..default()
                    },
                ),
                action_button("Start", StartButton),
                action_button("Speedup 2x", Speedup2xButton),
                action_button("Save", SaveButton),
                action_button("History", HistoryButton),
            ]
        )],
    ));
}

fn action_button<M: Component>(label: &str, marker: M) -> impl Bundle {
    (
        Button,
        marker,
        BackgroundColor(NORMAL_BUTTON),
        Node {
            padding: UiRect::all(px(8.0)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        Text::new(label),
        TextFont {
            font_size: 16.0,
            ..default()
        },
    )
}

fn button_system(
    mut q: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, mut bg) in &mut q {
        *bg = match interaction {
            Interaction::Pressed => PRESSED_BUTTON.into(),
            Interaction::Hovered => HOVERED_BUTTON.into(),
            Interaction::None => NORMAL_BUTTON.into(),
        };
    }
}

fn on_start_button_click(
    q: Query<&Interaction, (Changed<Interaction>, With<Button>, With<StartButton>)>,
    mut timer: ResMut<GlobalTimer>,
    mut ai_state: ResMut<NextState<AiState>>,
) {
    for interaction in &q {
        if *interaction == Interaction::Pressed {
            info!("Start pressed");
            ai_state.set(AiState::Runing);
            timer.1 = true;
        }
    }
}

fn on_speed_button_click(
    mut q: Query<
        (&Interaction, &mut Text),
        (Changed<Interaction>, With<Button>, With<Speedup2xButton>),
    >,
    mut timer: ResMut<GlobalTimer>,
    mut speed: ResMut<SpeedMultiplier>,
) {
    for (interaction, mut text) in &mut q {
        if *interaction != Interaction::Pressed {
            continue;
        }
        if speed.0 > 10.0 {
            speed.0 = 1.0
        } else {
            speed.0 *= 2.0;
        }

        timer.0.set_duration(std::time::Duration::from_secs_f32(
            (1.0 / speed.0).max(0.01),
        ));
        text.0 = format!("Speedup {}x", speed.0 as u32);
    }
}

fn on_save_button_click(
    q: Query<&Interaction, (Changed<Interaction>, With<Button>, With<SaveButton>)>,
    save: Res<SaveGamePath>,
    world: Res<GameWorld>,
    history: Res<History>,
) {
    for interaction in &q {
        if *interaction == Interaction::Pressed {
            let json = serde_json::to_string(&world.provinces).unwrap_or_else(|e| {
                error!("Coudlnt create json for province: {}", e);
                String::new()
            });

            let path = "assets\\".to_string() + &save.vec_provinces;
            fs::write(&path, json).unwrap_or_else(|e| {
                error!("Coudlnt save provinces in [{}] : {e}", &path);
            });

            let json = serde_json::to_string(&world.countries).unwrap_or_else(|e| {
                error!("Coudlnt create json for province: {}", e);
                String::new()
            });

            let path = "assets\\".to_string() + &save.vec_country;
            fs::write(&path, json).unwrap_or_else(|e| {
                error!("Coudlnt save provinces in [{}] : {e}", &path);
            });

            history.save();
        }
    }
}

fn on_history_button_click(
    q: Query<&Interaction, (Changed<Interaction>, With<Button>, With<HistoryButton>)>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for interaction in &q {
        if *interaction == Interaction::Pressed {
            game_state.set(GameState::History);
        }
    }
}
