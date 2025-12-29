use super::resources::Map;
use bevy::{
    camera::Viewport, math::ops::powf, prelude::*,
    sprite_render::AlphaMode2d,
};

use super::map_data::MapData;

pub struct MapDisplayPlugin;

impl Plugin for MapDisplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, setup)
            .add_systems(Update, select_province)
            .add_systems(Update, controls);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    map_data: Res<MapData>,
    asset_server: Res<AssetServer>,
    window: Single<&Window>,

    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let window_size = window.resolution.physical_size();

    commands.spawn((
        Camera2d,
        Camera {
            viewport: Some(Viewport {
                physical_position: uvec2(0, 0),
                physical_size: window_size,
                ..default()
            }),
            ..default()
        },
    ));

    let texture_handle = asset_server.load(map_data.get_path_in_assets());
    let mesh_handle = meshes.add(Rectangle::from_size(Vec2::new(
        map_data.width as f32,
        map_data.height as f32,
    )));

    commands.spawn((
        Mesh2d(mesh_handle.clone()),
        MeshMaterial2d(materials.add(ColorMaterial {
            alpha_mode: AlphaMode2d::Opaque,
            texture: Some(texture_handle.clone()),
            ..default()
        })),
    ));
}

fn select_province(
    camera_query: Single<(&Camera, &GlobalTransform)>,
    window: Single<&Window>,
    map: Res<Map>,
) {
    let (camera, camera_transform) = *camera_query;

    if let Some(cursor_position) = window.cursor_position()
        && let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_position)
    {
        //println!("{:?}", map.get_color(world_pos.x, world_pos.y));
    }
}

fn controls(
    camera_query: Single<(&mut Camera, &mut Transform, &mut Projection)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time<Fixed>>,
) {
    let (_, mut transform, mut projection) = camera_query.into_inner();

    if let Projection::Orthographic(projection2d) = &mut *projection {
        if input.pressed(KeyCode::Comma) {
            projection2d.scale *= powf(4.0f32, time.delta_secs());
        }

        if input.pressed(KeyCode::Period) {
            projection2d.scale *= powf(0.25f32, time.delta_secs());
        }

        let fspeed = 600.0 * time.delta_secs() * projection2d.scale;

        if input.pressed(KeyCode::ArrowUp) {
            transform.translation.y -= fspeed;
        }
        if input.pressed(KeyCode::ArrowDown) {
            transform.translation.y += fspeed;
        }
        if input.pressed(KeyCode::ArrowLeft) {
            transform.translation.x += fspeed;
        }
        if input.pressed(KeyCode::ArrowRight) {
            transform.translation.x -= fspeed;
        }
    }
}


