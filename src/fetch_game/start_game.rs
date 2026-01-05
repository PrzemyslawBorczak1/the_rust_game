use super::load_assets;
use bevy::{app::PluginGroupBuilder, prelude::*};
use to_delete::*;
pub struct LoadGamePlugin;

impl PluginGroup for LoadGamePlugin {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(SkipToLoad)
            .add(load_assets::LoadAssetsPlugin)
    }
}

//todo delete after debugin
mod to_delete {
    use super::*;

    use crate::data::{FetchGamePath, GameState, SaveGamePath};

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
    ) {
        set_data(fetch, save);
        game_state.set(GameState::LoadGame);
    }

    // todo delete
    fn set_data(mut fetch: ResMut<FetchGamePath>, mut save: ResMut<SaveGamePath>) {
        *fetch = FetchGamePath {
            id_texture: "map_id.png".to_string(),
            province_texture: "map_id.png".to_string(),
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
