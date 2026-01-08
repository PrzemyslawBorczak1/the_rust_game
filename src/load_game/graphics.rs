use crate::data::GameState;
use crate::data::{GameWorld, resources::*};
use crate::load_game::finish::LoadingState;
use bevy::prelude::*;

#[derive(States, Debug, Hash, Eq, PartialEq, Clone, Default)]
pub enum LoadingGraphics {
    Show,
    #[default]
    Disabled,
}

#[derive(Component)]
struct CheckTimer(Timer, bool);

pub struct LoadingGraphicsPlugin;

impl Plugin for LoadingGraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<LoadingGraphics>()
            .add_systems(OnEnter(GameState::LoadGame), loading_screen_setup)
            .add_systems(Update, check.run_if(in_state(LoadingGraphics::Show)));
    }
}

fn loading_screen_setup(mut commands: Commands) {
    commands.spawn((DespawnOnExit(LoadingGraphics::Show), Camera2d::default()));

    commands.spawn((
        DespawnOnExit(LoadingGraphics::Show),
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,

            ..default()
        },
        children![(
            Text::new("Loading"),
            TextFont {
                font_size: 67.0,
                ..default()
            },
            Node {
                margin: UiRect::all(px(50)),
                ..default()
            },
        ),],
    ));

    commands.spawn((
        DespawnOnExit(LoadingGraphics::Show),
        CheckTimer(Timer::from_seconds(1.0, TimerMode::Once), false),
    ));
}

fn check(
    time: Res<Time>,
    mut timer: Single<&mut CheckTimer>,
    mut state_menu: ResMut<NextState<LoadingGraphics>>,
    mut game_state: ResMut<NextState<GameState>>,
    state_loading: Res<State<LoadingState>>,
) {
    if timer.0.tick(time.delta()).is_finished() {
        timer.1 = true;
    }

    if !timer.1 {
        return;
    }

    if *state_loading.get() != LoadingState::Finished {
        return;
    }

    state_menu.set(LoadingGraphics::Disabled);
    game_state.set(GameState::Game);
}
