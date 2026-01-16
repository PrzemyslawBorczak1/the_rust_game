use super::super::common::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct InterfaceRightPanel;

#[derive(Component, Debug, Default, Clone, Copy)]
pub struct HideUiButton;
#[derive(Component)]
pub struct MapModeLabelText;

#[derive(Component, Debug, Default, Clone, Copy)]
pub struct SetPoliticalButton;

#[derive(Component, Debug, Default, Clone, Copy)]
pub struct SetGeographicalButton;

#[derive(Component)]
pub struct MessageLog;

pub fn right_panel() -> impl Bundle {
    (
        InterfaceRightPanel,
        Node {
            width: px(320),
            height: percent(100),
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(px(10)),
            border: UiRect::all(px(1)),
            align_items: AlignItems::FlexStart,
            ..default()
        },
        BackgroundColor(PANEL_BG),
        BorderColor::all(PANEL_BORDER),
        children![
            hide_ui_button(),
            map_draw_type_title(),
            map_mode_label(),
            map_mode_buttons_container(),
            message_log(),
        ],
    )
}

fn hide_ui_button() -> impl Bundle {
    (
        Button,
        HideUiButton,
        BackgroundColor(NORMAL_BUTTON),
        Node {
            padding: UiRect::all(px(8)),
            margin: UiRect::bottom(px(10)),
            ..default()
        },
        children![(
            Text::new("Hide UI (I)"),
            TextFont {
                font_size: 16.0,
                ..default()
            },
        )],
    )
}

fn map_draw_type_title() -> impl Bundle {
    (
        Text::new("Map Draw Type"),
        TextFont {
            font_size: 22.0,
            ..default()
        },
    )
}

fn map_mode_label() -> impl Bundle {
    (
        Text::new("Selected: Political"),
        TextFont {
            font_size: 16.0,
            ..default()
        },
        Node {
            margin: UiRect::bottom(px(8)),
            ..default()
        },
        MapModeLabelText,
    )
}

fn map_mode_buttons_container() -> impl Bundle {
    (
        Node {
            width: percent(100),
            flex_direction: FlexDirection::Column,
            row_gap: px(8),
            ..default()
        },
        children![
            big_button("Geographical", SetGeographicalButton),
            big_button("Political", SetPoliticalButton),
        ],
    )
}

fn big_button<M: Component>(label: &str, marker: M) -> impl Bundle {
    (
        Button,
        marker,
        BackgroundColor(NORMAL_BUTTON),
        Node {
            width: percent(100),
            padding: UiRect::all(px(10)),
            ..default()
        },
        children![(
            Text::new(label),
            TextFont {
                font_size: 16.0,
                ..default()
            },
        )],
    )
}

fn message_log() -> impl Bundle {
    (
        Text::new("Atack something"),
        TextFont {
            font_size: 16.0,
            ..default()
        },
        InterfaceText { base_size: 16.0 },
        TextColor(Color::srgb(0.0, 1.0, 0.0)),
        MessageLog,
    )
}
