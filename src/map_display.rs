use super::resources::Map;
use bevy::image::ImageSampler;
use bevy::render::render_resource::*;
use bevy::shader::ShaderRef;
use bevy::sprite_render::{Material2d, Material2dPlugin};
use bevy::{camera::Viewport, math::ops::powf, prelude::*, sprite_render::AlphaMode2d};

use super::map_data::MapData;

#[derive(Resource)]
pub struct ProvinceMapHandle(pub Handle<ProvinceMapMaterial>);

pub struct MapDisplayPlugin;

impl Plugin for MapDisplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<ProvinceMapMaterial>::default())
            .add_systems(PostStartup, setup)
            .add_systems(Update, configure_textures)
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
    mut province_materials: ResMut<Assets<ProvinceMapMaterial>>,
    mut images: ResMut<Assets<Image>>,
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

    let map_texture: Handle<Image> = asset_server.load(map_data.get_path_in_assets());
    let id_texture: Handle<Image> = asset_server.load(map_data.get_path_in_assets());

    println!("Map dimensions: {}x{}", map_data.width, map_data.height);

    let mesh_handle = meshes.add(Rectangle::from_size(Vec2::new(
        map_data.width as f32,
        map_data.height as f32,
    )));

    let material_handle = province_materials.add(ProvinceMapMaterial {
        map_texture: map_texture.clone(),
        id_texture: id_texture.clone(),
        selected_color: Vec4::new(0.0, 0.0, 0.0, 1.0), // No province selected initially
    });

    commands.insert_resource(ProvinceMapHandle(material_handle.clone()));

    commands.spawn((Mesh2d(mesh_handle), MeshMaterial2d(material_handle)));
}

fn configure_textures(
    material_handle: Option<Res<ProvinceMapHandle>>,
    materials: Res<Assets<ProvinceMapMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    if let Some(handle) = material_handle {
        if let Some(material) = materials.get(&handle.0) {
            // Configure map texture
            if let Some(image) = images.get_mut(&material.map_texture) {
                image.sampler = ImageSampler::nearest();
                image.texture_descriptor.format = TextureFormat::Rgba8Unorm;
            }
            // Configure id texture - use linear color space for exact color matching
            if let Some(image) = images.get_mut(&material.id_texture) {
                image.sampler = ImageSampler::nearest();
                image.texture_descriptor.format = TextureFormat::Rgba8Unorm;
            }
        }
    }
}

fn select_province(
    camera_query: Single<(&Camera, &GlobalTransform)>,
    window: Single<&Window>,
    map: Res<Map>,
    material_handle: Res<ProvinceMapHandle>,
    mut materials: ResMut<Assets<ProvinceMapMaterial>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
) {
    let (camera, camera_transform) = *camera_query;

    if mouse_input.just_pressed(MouseButton::Left) {
        if let Some(cursor_position) = window.cursor_position() {
            println!("Cursor position: {:?}", cursor_position);
            if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_position) {
                println!("World position: {:?}", world_pos);
                if let Some(province_color) = map.get_color(world_pos.x, world_pos.y) {
                    let color_vec = Vec4::new(
                        province_color[0] as f32 / 255.0,
                        province_color[1] as f32 / 255.0,
                        province_color[2] as f32 / 255.0,
                        1.0,
                    );
                    println!(
                        "Selected province RGB: {:?}, shader color: {:?}",
                        province_color, color_vec
                    );

                    // Update material to highlight selected province in white
                    if let Some(material) = materials.get_mut(&material_handle.0) {
                        material.selected_color = color_vec;
                        println!("Material updated with color: {:?}", color_vec);
                    } else {
                        println!("ERROR: Could not find material!");
                    }
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

#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
pub struct ProvinceMapMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub map_texture: Handle<Image>,

    #[texture(2)]
    #[sampler(3)]
    pub id_texture: Handle<Image>,

    #[uniform(4)]
    pub selected_color: Vec4,
}

impl Material2d for ProvinceMapMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/province.wgsl".into()
    }
}
