use crate::data::{FetchGamePath, GameState, SaveGamePath};
use bevy::prelude::*;
use seters::*;
use ui::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), menu_setup)
            .init_state::<MenuState>()
            .init_resource::<FetchGamePath>()
            .init_resource::<SaveGamePath>()
            .add_systems(OnEnter(MenuState::Main), main_menu)
            .add_systems(
                Update,
                (button_system, on_click).run_if(in_state(GameState::Menu)),
            )
            .add_systems(OnEnter(MenuState::Load), load)
            .add_systems(OnEnter(MenuState::NewGame), new_game);
    }
}

fn menu_setup(mut commands: Commands, mut state: ResMut<NextState<MenuState>>) {
    commands.spawn((DespawnOnExit(GameState::Menu), Camera2d::default()));
    state.set(MenuState::Main);
}

mod ui {
    use super::*;
    const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
    const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
    const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

    #[derive(Component)]
    pub enum MenuButtonAction {
        NewGame,
        Load,
        Quit,
    }

    #[derive(States, Debug, Hash, Eq, PartialEq, Clone, Default)]
    pub enum MenuState {
        Main,
        NewGame,
        Load,
        #[default]
        Disabled,
    }

    pub fn main_menu(mut commands: Commands) {
        commands.spawn((
            DespawnOnExit(MenuState::Main),
            Node {
                width: percent(100),
                height: percent(100),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,

                ..default()
            },
            children![(
                Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                },
                children![
                    (
                        Text::new("Bevy Game Menu UI"),
                        TextFont {
                            font_size: 67.0,
                            ..default()
                        },
                        Node {
                            margin: UiRect::all(px(50)),
                            ..default()
                        },
                    ),
                    (
                        Button,
                        MenuButtonAction::NewGame,
                        children![(), (Text::new("New Game"),),]
                    ),
                    (
                        Button,
                        MenuButtonAction::Load,
                        children![(), (Text::new("Load"),),]
                    ),
                    (
                        Button,
                        MenuButtonAction::Quit,
                        children![(), (Text::new("Quit"),),]
                    ),
                ]
            ),],
        ));
    }

    pub fn button_system(
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

    pub fn on_click(
        interaction_query: Query<
            (&Interaction, &MenuButtonAction),
            (Changed<Interaction>, With<Button>),
        >,
        mut app_exit_writer: MessageWriter<AppExit>,
        mut menu_state: ResMut<NextState<MenuState>>,
    ) {
        for part in interaction_query {
            if *part.0 == Interaction::Pressed {
                match *part.1 {
                    MenuButtonAction::Quit => {
                        app_exit_writer.write(AppExit::Success);
                    }
                    MenuButtonAction::Load => menu_state.set(MenuState::Load),
                    MenuButtonAction::NewGame => menu_state.set(MenuState::NewGame),
                };
            }
        }
    }
}
mod seters {
    use super::*;

    pub fn load(
        mut game_state: ResMut<NextState<GameState>>,
        fetch: ResMut<FetchGamePath>,
        save: ResMut<SaveGamePath>,
    ) {
        set_data(fetch, save);
        game_state.set(GameState::LoadGame);
    }

    pub fn new_game(
        mut game_state: ResMut<NextState<GameState>>,
        fetch: ResMut<FetchGamePath>,
        save: ResMut<SaveGamePath>,
    ) {
        set_data(fetch, save);
        game_state.set(GameState::LoadGame);
    }

    // todo delete
    fn set_data(mut fetch: ResMut<FetchGamePath>, mut save: ResMut<SaveGamePath>) {
        *fetch = FetchGamePath {
            id_texture: "map_id.png".to_string(),
            province_texture: "map_id.png".to_string(),
            vec_provinces: "json".to_string(),
            vec_country: "json".to_string(),
        };

        *save = SaveGamePath {
            id_texture: "id_save1.png".to_string(),
            province_texture: "texture_save1.png".to_string(),
            vec_provinces: "json".to_string(),
            vec_country: "json".to_string(),
        }
    }
}
