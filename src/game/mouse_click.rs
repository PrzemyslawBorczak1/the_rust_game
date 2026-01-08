use bevy::prelude::*;

use crate::data::{GPUMaterial, GameState, GameWorld};

pub struct MouseClickPlugin;

impl Plugin for MouseClickPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (select_province).run_if(in_state(GameState::Game)));
    }
}

fn select_province(
    camera_query: Single<(&Camera, &GlobalTransform)>,
    window: Single<&Window>,
    mouse_input: Res<ButtonInput<MouseButton>>,

    map: Res<GameWorld>,
    mut materials: ResMut<Assets<GPUMaterial>>,
) {
    let (camera, camera_transform) = *camera_query;

    if mouse_input.just_pressed(MouseButton::Left) {
        if let Some(cursor_position) = window.cursor_position() {
            println!("Cursor position: {:?}", cursor_position);
            if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_position) {
                println!("World position: {:?}", world_pos);
                map.select_province(world_pos.x, world_pos.y, &mut *materials);
            } else {
                println!("Failed to convert cursor to world position");
            }
        } else {
            println!("No cursor position");
        }
    }
}
