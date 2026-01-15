use bevy::prelude::*;
use shared::resources::GameWorld;

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameWorld>();
    }
}
