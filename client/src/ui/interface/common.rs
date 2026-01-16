use bevy::prelude::*;

pub const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

pub const PANEL_BG: Color = Color::srgba(0.05, 0.05, 0.05, 0.88);
pub const PANEL_BORDER: Color = Color::srgba(1.0, 1.0, 1.0, 0.08);

#[derive(States, Debug, Hash, Eq, PartialEq, Clone, Default)]
pub enum InterfaceState {
    Visibile,
    #[default]
    Hidden,
}

#[derive(Component)]
pub struct InterfaceRoot;

#[derive(Component, Debug, Clone, Copy)]
pub struct InterfaceText {
    pub base_size: f32,
}

#[derive(States, Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum AttackState {
    #[default]
    NoAtack,
    Choose,
}

#[derive(Resource)]
pub struct ActiveProvince(pub u32);

#[derive(Message)]
pub struct Refresch;
