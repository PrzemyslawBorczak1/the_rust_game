use bevy::prelude::*;

use crate::ui::GPUMaterialHandle;

use super::gpu::GPUMaterial;
use shared::resources::GameWorld;

pub struct MouseClickPlugin;

impl Plugin for MouseClickPlugin {
    fn build(&self, app: &mut App) {}
}
