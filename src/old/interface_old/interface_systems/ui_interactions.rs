use bevy::prelude::*;

use bevy_ecs::query::QueryFilter;

use super::super::common::*;

pub fn interface_button_colors(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut background_color) in &mut interaction_query {
        *background_color = match interaction {
            Interaction::Pressed => PRESSED_BUTTON.into(),
            Interaction::Hovered => HOVERED_BUTTON.into(),
            Interaction::None => NORMAL_BUTTON.into(),
        }
    }
}

fn any_pressed<F: QueryFilter>(q: &Query<&Interaction, F>) -> bool {
    q.iter().any(|i| *i == Interaction::Pressed)
}

pub fn emit_toggle_ui_from_button(
    q: Query<&Interaction, (Changed<Interaction>, With<Button>, With<ToggleUiButton>)>,
    mut out: MessageWriter<ToggleUiEvent>,
) {
    if any_pressed(&q) {
        out.write(ToggleUiEvent);
    }
}

pub fn emit_show_province_from_button(
    q: Query<&Interaction, (Changed<Interaction>, With<Button>, With<ShowProvinceButton>)>,
    mut out: MessageWriter<SetLeftPanelViewEvent>,
) {
    if any_pressed(&q) {
        out.write(SetLeftPanelViewEvent(LeftPanelView::Province));
    }
}

pub fn emit_show_country_from_button(
    q: Query<&Interaction, (Changed<Interaction>, With<Button>, With<ShowCountryButton>)>,
    mut out: MessageWriter<SetLeftPanelViewEvent>,
) {
    if any_pressed(&q) {
        out.write(SetLeftPanelViewEvent(LeftPanelView::Country));
    }
}

pub fn emit_set_political_from_button(
    q: Query<&Interaction, (Changed<Interaction>, With<Button>, With<SetPoliticalButton>)>,
    mut out: MessageWriter<SetMapDrawModeEvent>,
) {
    if any_pressed(&q) {
        out.write(SetMapDrawModeEvent(MapDrawMode::Political));
    }
}

pub fn emit_set_geographical_from_button(
    q: Query<
        &Interaction,
        (
            Changed<Interaction>,
            With<Button>,
            With<SetGeographicalButton>,
        ),
    >,
    mut out: MessageWriter<SetMapDrawModeEvent>,
) {
    if any_pressed(&q) {
        out.write(SetMapDrawModeEvent(MapDrawMode::Geographical));
    }
}

pub fn interface_toggle_keyboard(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut out: MessageWriter<ToggleUiEvent>,
) {
    if !keyboard.just_pressed(KeyCode::KeyI) {
        return;
    }

    out.write(ToggleUiEvent);
}
