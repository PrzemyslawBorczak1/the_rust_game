use bevy::prelude::*;
use bevy::render::render_resource::AsBindGroup;
use bevy::render::storage::ShaderStorageBuffer;
use bevy::shader::ShaderRef;
use bevy::sprite_render::Material2d;

use bevy::{prelude::*, render::render_resource::ShaderType};
use serde;
use serde::*;

use bevy::reflect::TypePath;
pub const NO_SELECTED_ID: u32 = 213767;
pub const GEOGRAPHICAL_DRAW: u32 = 0;
pub const POLITICAL_DRAW: u32 = 1;

#[derive(Debug, Default, Clone, Serialize, Deserialize, ShaderType)]
pub struct CountryGpu {
    pub color: Vec4,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, ShaderType)]
pub struct ProvinceGpu {
    pub owner_id: u32,
    pub terrain_type: u32,
}

#[derive(AsBindGroup, Debug, Clone, Asset, TypePath)]
pub struct GPUMaterial {
    #[storage(1, read_only)]
    pub id: Handle<ShaderStorageBuffer>,

    #[uniform(2)]
    pub width: u32,

    #[uniform(3)]
    pub height: u32,

    #[storage(4, read_only)]
    pub provinces: Handle<ShaderStorageBuffer>,

    #[storage(5, read_only)]
    pub countries: Handle<ShaderStorageBuffer>,

    #[uniform(6)]
    pub selected_id: u32,

    #[uniform(7)]
    pub draw_type: u32,
    // #[texture(7)]
    // #[sampler(8)]
    // pub provinces_texture: Handle<Image>,
}

impl Material2d for GPUMaterial {
    fn fragment_shader() -> ShaderRef {
        "shader.wgsl".into()
    }
}

#[derive(Resource, Default, Debug, Clone)]
pub struct GPUMaterialHandle(pub Handle<GPUMaterial>);
