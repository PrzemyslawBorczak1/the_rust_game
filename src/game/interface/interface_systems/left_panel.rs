use bevy::prelude::*;

use crate::data::{GameWorld, gpu::*};

use super::super::common::*;

pub fn interface_update_left_panel(
    view: Res<State<LeftPanelView>>,
    world: Option<Res<GameWorld>>,
    materials: Res<Assets<GPUMaterial>>,
    mut texts: ParamSet<(
        Query<&mut Text, With<LeftPanelTitleText>>,
        Query<&mut Text, With<LeftPanelMetaText>>,
    )>,
    mut flag_slot: Query<&mut Visibility, With<CountryFlagSlot>>,
) {
    let view = *view.get();

    let Some(mut flag_vis) = flag_slot.iter_mut().next() else {
        return;
    };

    let title_string = match view {
        LeftPanelView::Province => "Province Data".to_string(),
        LeftPanelView::Country => "Country Data".to_string(),
    };

    let Some(world) = world.as_deref() else {
        let meta_string = "No world loaded".to_string();
        *flag_vis = Visibility::Hidden;

        if let Some(mut title_text) = texts.p0().iter_mut().next() {
            *title_text = Text::new(title_string);
        }
        if let Some(mut meta_text) = texts.p1().iter_mut().next() {
            *meta_text = Text::new(meta_string);
        }
        return;
    };

    let selected_province_id = materials
        .get(world.gpu.id())
        .map(|m| m.selected_id)
        .unwrap_or(NO_SELECTED_ID);

    let meta_string = match view {
        LeftPanelView::Province => {
            *flag_vis = Visibility::Hidden;

            let placeholder_owner_country_id = (selected_province_id % 7).max(1);
            let placeholder_terrain = match selected_province_id % 4 {
                0 => "Flat",
                1 => "Forest",
                2 => "Mountain",
                _ => "Water",
            };

            format!(
                "Selected province id: {}\nName: Province {}\nOwner country id: {}\nTerrain: {}\nPopulation: {}\nDevelopment: {}\nTax: {}\nManpower: {}",
                selected_province_id,
                selected_province_id,
                placeholder_owner_country_id,
                placeholder_terrain,
                123_456 + selected_province_id * 3,
                10 + (selected_province_id % 15),
                3 + (selected_province_id % 5),
                1 + (selected_province_id % 7)
            )
        }
        LeftPanelView::Country => {
            *flag_vis = Visibility::Visible;

            let placeholder_country_id = (selected_province_id % 7).max(1);
            let name = match placeholder_country_id {
                1 => "Placeholderland",
                2 => "Testovia",
                3 => "Rust Empire",
                4 => "Bevy Kingdom",
                5 => "Iron Republic",
                6 => "Northern Union",
                _ => "Southern League",
            };

            format!(
                "Selected province id: {}\nCountry id: {}\nName: {}\nGovernment: Monarchy (placeholder)\nRuler: King Placeholder I\nStability: +{}\nTreasury: {}\nManpower: {}\nDiplomacy: Neutral",
                selected_province_id,
                placeholder_country_id,
                name,
                1 + (selected_province_id % 3),
                1_000 + (selected_province_id as i32 * 7),
                50_000 + selected_province_id * 11
            )
        }
    };

    if let Some(mut title_text) = texts.p0().iter_mut().next() {
        *title_text = Text::new(title_string);
    }
    if let Some(mut meta_text) = texts.p1().iter_mut().next() {
        *meta_text = Text::new(meta_string);
    }
}
