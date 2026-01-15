use std::default;

use super::super::common::InterfaceState;
use super::super::desgin::left_panel::*;
use crate::ui::NO_SELECTED_ID;
use bevy::prelude::*;
use shared::resources::GameWorld;

#[derive(States, Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum LeftPanelView {
    #[default]
    Province,
    Country,
}

#[derive(Resource)]
pub struct ActiveProvince(pub u32);

#[derive(Message)]
pub struct Refresch;

pub struct LeftPanelFunctionalityPlugin;

impl Plugin for LeftPanelFunctionalityPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<Refresch>()
            .insert_resource(ActiveProvince(NO_SELECTED_ID))
            .init_state::<LeftPanelView>()
            .add_systems(
                Update,
                (
                    (on_country_button_click, on_province_button_click)
                        .run_if(in_state(InterfaceState::Visibile)),
                    on_refresh,
                ),
            )
            .add_systems(
                OnEnter(LeftPanelView::Country),
                (set_country_view, set_text, set_flag).chain(),
            )
            .add_systems(
                OnEnter(LeftPanelView::Province),
                (set_province_view, set_text).chain(),
            );
    }
}

fn on_country_button_click(
    q: Query<&Interaction, (Changed<Interaction>, With<Button>, With<ShowCountryButton>)>,
    mut next: ResMut<NextState<LeftPanelView>>,
) {
    for interaction in q {
        if *interaction == Interaction::Pressed {
            next.set(LeftPanelView::Country);
        }
    }
}

fn on_province_button_click(
    q: Query<&Interaction, (Changed<Interaction>, With<Button>, With<ShowProvinceButton>)>,
    mut next: ResMut<NextState<LeftPanelView>>,
) {
    for interaction in q {
        if *interaction == Interaction::Pressed {
            next.set(LeftPanelView::Province);
        }
    }
}

fn set_country_view(mut commands: Commands, q_root: Query<Entity, With<LeftPanelBody>>) {
    if let Ok(root) = q_root.single() {
        commands.entity(root).despawn_children();

        commands.entity(root).with_children(|parent| {
            parent.spawn(country_data());
        });
    }
}

fn set_province_view(mut commands: Commands, q_root: Query<Entity, With<LeftPanelBody>>) {
    if let Ok(root) = q_root.single() {
        commands.entity(root).despawn_children();

        commands.entity(root).with_children(|parent| {
            parent.spawn(province_meta_text());
        });
    }
}

fn set_text(
    mut q: Query<(
        &mut Text,
        Option<&ProvinceMetaText>,
        Option<&CountryMetaText>,
    )>,
    world: Res<GameWorld>,
    active_province: Res<ActiveProvince>,
) {
    println!("Active prov: {}", active_province.0);
    for (mut text, is_province, is_country) in &mut q {
        if is_province.is_some() {
            text.0 = format!("{:#?}", world.get_province(active_province.0));
        }
        if is_country.is_some() {
            let str = format!("{:#?}", world.get_country(active_province.0));
            println!("{str}");
            text.0 = str;
        }
    }
}

fn on_refresh(
    q: Query<(
        &mut Text,
        Option<&ProvinceMetaText>,
        Option<&CountryMetaText>,
    )>,
    world: Res<GameWorld>,
    mut reader: MessageReader<Refresch>,
    active_province: Res<ActiveProvince>,

    mut q_flag: Query<(&mut ImageNode, Option<&CountryMetaText>)>,
    asset_server: Res<AssetServer>,
) {
    if reader.read().next().is_none() {
        return;
    }

    // the same function as below :(
    for (mut img, _) in &mut q_flag {
        match world.get_country(active_province.0) {
            Some(c) => img.image = asset_server.load(&c.flag_path),
            None => img.image = Handle::<Image>::default(),
        }
    }

    set_text(q, world, active_province);
}

fn set_flag(
    mut q: Query<(&mut ImageNode, Option<&CountryMetaText>)>,
    world: Res<GameWorld>,
    asset_server: Res<AssetServer>,
    active_province: Res<ActiveProvince>,
) {
    for (mut img, _) in &mut q {
        let path = match world.get_country(active_province.0) {
            Some(c) => &c.flag_path,
            None => return,
        };
        let new = asset_server.load(path);
        println!("Handle {:?}", new);
        img.image = new;
    }
}
