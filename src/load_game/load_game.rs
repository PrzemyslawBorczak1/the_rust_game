use crate::load_game::{finish::FinishLoadingPlugin, gpu::AddGPUPlugin};
use crate::load_game::graphics::LoadingGraphicsPlugin;

use super::load_resource::*;
use bevy::{app::PluginGroupBuilder, prelude::*};
use to_delete::*;
pub struct LoadGamePlugin;

impl PluginGroup for LoadGamePlugin {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(SkipToLoad)
            .add(LoadingGraphicsPlugin)
            .add(LoadVecProvincePlugin)
            .add(LoadVecCountryPlugin)
            .add(LoadIdMapPlugin)
            .add(LoadTexturesPlugin)
            .add(AddGPUPlugin)
            .add(FinishLoadingPlugin)
    }
}

//todo delete after debugin
mod to_delete {
    use super::*;

    use crate::{
        data::{FetchGamePath, GameState, SaveGamePath},
        load_game::graphics::LoadingGraphics,
        menu::MenuState,
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
            province_texture: "textures.json".to_string(),
            vec_provinces: "provinces.json".to_string(),
            vec_country: "countries.json".to_string(),
        };

        *save = SaveGamePath {
            id_texture: "id_save1.png".to_string(),
            province_texture: "texture_save1.png".to_string(),
            vec_provinces: "provinces.json".to_string(),
            vec_country: "countries.json".to_string(),
        }
    }
}
