use bevy::{
    camera::Viewport, math::ops::powf, prelude::*,
    sprite_render::Material2dPlugin, window::WindowResized,
};

use crate::data::{GameState, Map, Textures, TexturesHandle};
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<Textures>::default())
            .add_systems(
                OnEnter(GameState::Game),
                (game_setup, on_resize_system).chain(),
            )
            .add_systems(
                Update,
                (
                    controls,
                    select_province,
                    on_resize_system.run_if(on_message::<WindowResized>),
                )
                    .run_if(in_state(GameState::Game)),
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
    map: Res<Map>,
    time: Res<Time>,
) {
    let (camera, mut transform, mut projection) = camera_query.into_inner();

    if let Projection::Orthographic(projection2d) = &mut *projection {
        let delta = time.delta_secs();

        if input.pressed(KeyCode::Comma) {
            projection2d.scale *= powf(4.0f32, delta);
        }
        if input.pressed(KeyCode::Period) {
            projection2d.scale *= powf(0.25f32, delta);
        }

        let viewport_size = camera
            .viewport
            .as_ref()
            .map(|v| v.physical_size)
            .unwrap_or_else(|| UVec2::new(1, 1));

        // Zoom clamp: allow zooming IN as much as you want,
        // but cap zooming OUT so you can see the whole map + a tiny border.
        // (Prevents the map becoming "too small" with lots of outer space.)
        let usable_w_px = (viewport_size.x as f32).max(1.0);
        let usable_h_px = (viewport_size.y as f32).max(1.0);

        // Need the view (in world units) to be at least the map size in both axes.
        // view_world = viewport_px * scale  =>  scale >= map_world / usable_px
        let max_scale_x = map.width as f32 / usable_w_px;
        let max_scale_y = map.height as f32 / usable_h_px;
        let max_scale = max_scale_x.max(max_scale_y);
        projection2d.scale = projection2d.scale.min(max_scale);

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

        // Visible half extents in world units (scaled by zoom)
        let half_w = viewport_size.x as f32 * 0.5 * projection2d.scale;
        let half_h = viewport_size.y as f32 * 0.5 * projection2d.scale;

        let map_half_w = map.width as f32 * 0.5;
        let map_half_h = map.height as f32 * 0.5;

        // Clamp camera center so the view doesn’t leave the map
        let max_x = (map_half_w - half_w).max(0.0);
        let max_y = (map_half_h - half_h).max(0.0);

        transform.translation.x = transform.translation.x.clamp(-max_x, max_x);
        transform.translation.y = transform.translation.y.clamp(-max_y, max_y);
    }
}

fn on_resize_system(camera_query: Single<&mut Camera>, window: Single<&Window>) {
    let window_size = window.resolution.physical_size();
    let mut camera = camera_query.into_inner();

    if let Some(viewport) = &mut camera.viewport {
        viewport.physical_size = window_size;
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
