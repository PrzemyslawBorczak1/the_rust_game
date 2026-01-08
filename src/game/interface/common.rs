use bevy::prelude::*;

use crate::data::{GPUMaterial, GameWorld};

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum InterfaceUiSet {
    Input,
    Apply,
    Update,
}

pub const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

pub const PANEL_BG: Color = Color::srgba(0.05, 0.05, 0.05, 0.88);
pub const PANEL_BORDER: Color = Color::srgba(1.0, 1.0, 1.0, 0.08);

#[derive(Component)]
pub struct InterfaceRoot;

#[derive(Component)]
pub struct InterfaceLeftPanel;

#[derive(Component)]
pub struct InterfaceRightPanel;

#[derive(Component)]
pub struct LeftPanelBody;

#[derive(Component)]
pub struct CountryFlagSlot;

#[derive(Component)]
pub struct LeftPanelTitleText;

#[derive(Component)]
pub struct LeftPanelMetaText;

#[derive(Component)]
pub struct MapModeLabelText;

#[derive(Component, Debug, Clone, Copy)]
pub struct InterfaceText {
    pub base_size: f32,
}

#[derive(States, Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum LeftPanelView {
    #[default]
    Province,
    Country,
}

#[derive(Resource, Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum MapDrawMode {
    #[default]
    Political,
    Geographical,
}

#[derive(Component, Debug, Default, Clone, Copy)]
pub struct ToggleUiButton;

#[derive(Component, Debug, Default, Clone, Copy)]
pub struct ShowProvinceButton;

#[derive(Component, Debug, Default, Clone, Copy)]
pub struct ShowCountryButton;

#[derive(Component, Debug, Default, Clone, Copy)]
pub struct SetPoliticalButton;

#[derive(Component, Debug, Default, Clone, Copy)]
pub struct SetGeographicalButton;

#[derive(Message, Debug, Default, Clone, Copy)]
pub struct ToggleUiEvent;

#[derive(Message, Debug, Clone, Copy)]
pub struct SetLeftPanelViewEvent(pub LeftPanelView);

#[derive(Message, Debug, Clone, Copy)]
pub struct SetMapDrawModeEvent(pub MapDrawMode);

pub fn set_map_draw_mode(
    world: &GameWorld,
    materials: &mut Assets<GPUMaterial>,
    map_draw_mode: MapDrawMode,
) {
    if let Some(mat) = materials.get_mut(world.gpu.id()) {
        mat.draw_type = match map_draw_mode {
            MapDrawMode::Political => 0,
            MapDrawMode::Geographical => 1,
        };
    }
}
