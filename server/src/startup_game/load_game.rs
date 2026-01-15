use crate::startup_game::graphics::LoadingGraphicsPlugin;
use crate::startup_game::{finish::FinishLoadingPlugin, new_game::NewGamePlugin};

use super::load_resource::*;
use bevy::{app::PluginGroupBuilder, prelude::*};
use to_delete::*;
pub struct StartUpGamePlugin;

impl PluginGroup for StartUpGamePlugin {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            // .add(SkipToLoad)
            .add(LoadingGraphicsPlugin)
            .add(LoadVecProvincePlugin)
            .add(LoadVecCountryPlugin)
            .add(LoadIdMapPlugin)
            .add(FinishLoadingPlugin)
            .add(NewGamePlugin)
    }
}

//todo delete after debugin
mod to_delete {
    use super::*;

    use crate::{
        data::{FetchGamePath, GameState, SaveGamePath},
        menu::MenuState,
        startup_game::graphics::LoadingGraphics,
    };

    pub struct SkipToLoad;

    impl Plugin for SkipToLoad {
        fn build(&self, app: &mut App) {
            app.init_resource::<FetchGamePath>()
                .init_resource::<SaveGamePath>()
                .add_systems(Startup, skip);
        }
    }

    fn skip(
        fetch: ResMut<FetchGamePath>,
        save: ResMut<SaveGamePath>,
        mut game_state: ResMut<NextState<GameState>>,
        mut menu_state: ResMut<NextState<MenuState>>,
        mut graphics_state: ResMut<NextState<LoadingGraphics>>,
    ) {
        set_data(fetch, save);
        game_state.set(GameState::LoadGame);
        menu_state.set(MenuState::Disabled);
        graphics_state.set(LoadingGraphics::Show);
    }

    // todo delete
    fn set_data(mut fetch: ResMut<FetchGamePath>, mut save: ResMut<SaveGamePath>) {
        *fetch = FetchGamePath {
            id_texture: "map_nr.png".to_string(),
            vec_provinces: "provinces.json".to_string(),
            vec_country: "countries.json".to_string(),
        };

        *save = SaveGamePath {
            id_texture: "id_save1.png".to_string(),
            vec_provinces: "provinces.json".to_string(),
            vec_country: "countries.json".to_string(),
        }
    }
}
