use bevy::prelude::*;
use bevy::render::render_resource::AsBindGroup;
use bevy::shader::ShaderRef;
use bevy::sprite_render::Material2d;

use bevy::{prelude::*, reflect::TypePath};

#[derive(AsBindGroup, Debug, Clone, Asset, TypePath)]
pub struct Textures {
    #[texture(1)]
    #[sampler(2)]
    pub province_handle: Handle<Image>,

    #[texture(3)]
    #[sampler(4)]
    pub map_handle: Handle<Image>,

    #[uniform(5)]
    pub selected_color: Vec4,
}

impl Material2d for Textures {
    fn fragment_shader() -> ShaderRef {
        "shaders/shader.wgsl".into()
    }
}

#[derive(Resource, Default, Debug, Clone)]
pub struct TexturesHandle(pub Handle<Textures>);
