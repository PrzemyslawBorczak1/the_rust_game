use bevy::{math::ops::powf, prelude::*, window::WindowResized};

use shared::resources::GameWorld;

pub struct CameraMovementPlugin;

impl Plugin for CameraMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                controls,
                on_resize_system.run_if(on_message::<WindowResized>),
            ),
        );
    }
}

fn controls(
    camera_query: Single<(&mut Camera, &mut Transform, &mut Projection)>,

    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time<Fixed>>,
    map: Res<GameWorld>,
) {
    if map.width() == 0 || map.height() == 0 {
        return;
    }

    let (camera, mut transform, mut projection) = camera_query.into_inner();

    if let Projection::Orthographic(projection2d) = &mut *projection {
        let delta = time.delta_secs();

        if input.pressed(KeyCode::Comma) {
            projection2d.scale *= powf(4.0f32, delta);
        }
        if input.pressed(KeyCode::Period) {
            projection2d.scale *= powf(0.25f32, delta);
        }

        let viewport_size = match camera.viewport.as_ref().map(|v| v.physical_size) {
            None => {
                error!("No viewport in camera");
                return;
            }
            Some(x) => x,
        };

        let usable_w_px = (viewport_size.x as f32).max(1.0);
        let usable_h_px = (viewport_size.y as f32).max(1.0);

        let max_scale_x = map.width() as f32 / usable_w_px;
        let max_scale_y = map.height() as f32 / usable_h_px;
        let max_scale = max_scale_x.max(max_scale_y);
        projection2d.scale = projection2d.scale.min(max_scale);

        let fspeed = 600.0 * delta * projection2d.scale;

        if input.pressed(KeyCode::KeyW) {
            transform.translation.y += fspeed;
        }
        if input.pressed(KeyCode::KeyS) {
            transform.translation.y -= fspeed;
        }
        if input.pressed(KeyCode::KeyA) {
            transform.translation.x -= fspeed;
        }
        if input.pressed(KeyCode::KeyD) {
            transform.translation.x += fspeed;
        }

        let half_w = viewport_size.x as f32 * 0.5 * projection2d.scale;
        let half_h = viewport_size.y as f32 * 0.5 * projection2d.scale;

        let map_half_w = map.width() as f32 * 0.5;
        let map_half_h = map.height() as f32 * 0.5;

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
