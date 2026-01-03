use bevy::{
    camera::Viewport, math::ops::powf, prelude::*, render::render_resource::Texture,
    sprite_render::Material2dPlugin,
};

use crate::data::{GameState, Map, Textures, TexturesHandle};
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<Textures>::default())
            .add_systems(OnEnter(GameState::Game), game_setup)
            .add_systems(
                Update,
                (controls, select_province).run_if(in_state(GameState::Game)),
            );
    }
}

fn game_setup(
    mut commands: Commands,
    map: Res<Map>,
    mut meshes: ResMut<Assets<Mesh>>,
    texture: Res<TexturesHandle>,
    window: Single<&Window>,
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

    let mesh_handle = meshes.add(Rectangle::from_size(Vec2::new(
        map.width as f32,
        map.height as f32,
    )));

    commands.spawn((Mesh2d(mesh_handle), MeshMaterial2d(texture.0.clone())));
}

fn controls(
    camera_query: Single<(&mut Camera, &mut Transform, &mut Projection)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let (_, mut transform, mut projection) = camera_query.into_inner();

    if let Projection::Orthographic(projection2d) = &mut *projection {
        let delta = time.delta_secs();

        if input.pressed(KeyCode::Comma) {
            projection2d.scale *= powf(4.0f32, delta);
        }
        if input.pressed(KeyCode::Period) {
            projection2d.scale *= powf(0.25f32, delta);
        }

        let fspeed = 600.0 * delta * projection2d.scale;

        if input.pressed(KeyCode::ArrowUp) {
            transform.translation.y += fspeed;
        }
        if input.pressed(KeyCode::ArrowDown) {
            transform.translation.y -= fspeed;
        }
        if input.pressed(KeyCode::ArrowLeft) {
            transform.translation.x -= fspeed;
        }
        if input.pressed(KeyCode::ArrowRight) {
            transform.translation.x += fspeed;
        }
    }
}

// todo error
fn select_province(
    camera_query: Single<(&Camera, &GlobalTransform)>,
    window: Single<&Window>,
    mouse_input: Res<ButtonInput<MouseButton>>,

    map: Res<Map>,
    material_handle: Res<TexturesHandle>,
    mut materials: ResMut<Assets<Textures>>,
) {
    let (camera, camera_transform) = *camera_query;

    if mouse_input.just_pressed(MouseButton::Left) {
        if let Some(cursor_position) = window.cursor_position() {
            println!("Cursor position: {:?}", cursor_position);
            if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_position) {
                println!("World position: {:?}", world_pos);
                if let Some(province_color) = map.get_color(world_pos.x, world_pos.y) {
                    println!("Selected province RGBA: {:?}", province_color);
                    if let Some(material) = materials.get_mut(&material_handle.0) {
                        material.selected_color = Vec4::new(
                            province_color[0] as f32 / 255.0,
                            province_color[1] as f32 / 255.0,
                            province_color[2] as f32 / 255.0,
                            province_color[3] as f32 / 255.0,
                        );
                    } else {
                        println!("No province color found at position");
                    }
                } else {
                    println!("Failed to convert cursor to world position");
                }
            } else {
                println!("No cursor position");
            }
        }
    }
}
