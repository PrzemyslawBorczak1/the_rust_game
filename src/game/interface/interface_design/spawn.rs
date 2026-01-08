use bevy::prelude::*;

use crate::data::GameState;

use super::super::common::*;

pub fn spawn_interface(commands: &mut Commands) {
    commands.spawn((
        DespawnOnExit(GameState::Game),
        InterfaceRoot,
        Visibility::Visible,
        root_node(),
        Name::new("InterfaceRoot"),
        children![left_panel(), right_panel()],
    ));
}

fn root_node() -> Node {
    Node {
        width: percent(100),
        height: percent(100),
        justify_content: JustifyContent::SpaceBetween,
        align_items: AlignItems::Stretch,
        ..default()
    }
}

fn left_panel() -> impl Bundle {
    (
        InterfaceLeftPanel,
        Node {
            width: px(320),
            height: percent(100),
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(px(10)),
            border: UiRect::all(px(1)),
            ..default()
        },
        BackgroundColor(PANEL_BG),
        BorderColor::all(PANEL_BORDER),
        Name::new("InterfaceLeftPanel"),
        children![left_panel_header(), left_panel_body()],
    )
}

fn left_panel_header() -> impl Bundle {
    (
        Node {
            width: percent(100),
            flex_direction: FlexDirection::Column,
            row_gap: px(6),
            ..default()
        },
        children![
            (
                Text::new("Province Data"),
                TextFont {
                    font_size: 22.0,
                    ..default()
                },
                InterfaceText { base_size: 22.0 },
                LeftPanelTitleText,
            ),
            (
                Node {
                    width: percent(100),
                    flex_direction: FlexDirection::Row,
                    column_gap: px(8),
                    ..default()
                },
                children![

                action_button("Province", ShowProvinceButton),
                    action_button("Country", ShowCountryButton),
                ]
            ),
        ],
    )
}

fn left_panel_body() -> impl Bundle {
    (
        LeftPanelBody,
        Node {
            width: percent(100),
            flex_grow: 1.0,
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(px(8)),
            border: UiRect::all(px(1)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.25)),
        BorderColor::all(PANEL_BORDER),
        children![
            (
                CountryFlagSlot,
                Visibility::Hidden,
                Node {
                    width: px(96),
                    height: px(64),
                    border: UiRect::all(px(1)),
                    margin: UiRect::bottom(px(10)),
                    ..default()
                },
                BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.04)),
                BorderColor::all(PANEL_BORDER),
                children![(
                    Text::new("Flag"),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    InterfaceText { base_size: 14.0 },
                )]
            ),
            (
                Text::new(""),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                InterfaceText { base_size: 16.0 },
                LeftPanelMetaText,
            ),
        ],
    )
}

fn right_panel() -> impl Bundle {
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
        Name::new("InterfaceRightPanel"),
        children![
            (
                Button,
                ToggleUiButton,
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
                    InterfaceText { base_size: 16.0 },
                )]
            ),
            (
                Text::new("Map Draw Type"),
                TextFont {
                    font_size: 22.0,
                    ..default()
                },
                InterfaceText { base_size: 22.0 },
            ),
            (
                Text::new("Selected: Political"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                InterfaceText { base_size: 16.0 },
                Node {
                    margin: UiRect::bottom(px(8)),
                    ..default()
                },
                MapModeLabelText,
            ),
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
                ]
            ),
        ],
    )
}

fn action_button<M: Component>(label: &str, marker: M) -> impl Bundle {
    (
        Button,
        marker,
        BackgroundColor(NORMAL_BUTTON),
        Node {
            padding: UiRect::all(px(8)),
            ..default()
        },
        children![(
            Text::new(label),
            TextFont {
                font_size: 16.0,
                ..default()
            },
            InterfaceText { base_size: 16.0 },
        )],
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
            InterfaceText { base_size: 16.0 },
        )],
    )
}
