use bevy::prelude::*;
use bevy::render::render_resource::AsBindGroup;
use bevy::render::storage::ShaderStorageBuffer;
use bevy::shader::ShaderRef;
use bevy::sprite_render::Material2d;

use bevy::{prelude::*, reflect::TypePath};

#[derive(AsBindGroup, Debug, Clone, Asset, TypePath)]
pub struct GPUMap {
    #[texture(1)]
    #[sampler(2)]
    pub map_handle: Handle<Image>,

    #[storage(3, read_only)]
    pub id: Handle<ShaderStorageBuffer>,

    #[uniform(4)]
    pub width: u32,

    #[uniform(5)]
    pub height: u32,

    #[uniform(6)]
    pub selected_color: UVec4,
}

impl Material2d for GPUMap {
    fn fragment_shader() -> ShaderRef {
        "shaders/shader.wgsl".into()
    }
}

#[derive(Resource, Default, Debug, Clone)]
pub struct GPUMapHandle(pub Handle<GPUMap>);
