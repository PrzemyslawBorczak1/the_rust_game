use super::super::common::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct InterfaceLeftPanel;

#[derive(Component, Debug, Default, Clone, Copy)]
pub struct ShowProvinceButton;

#[derive(Component, Debug, Default, Clone, Copy)]
pub struct ShowCountryButton;

#[derive(Component)]
pub struct LeftPanelBody;

#[derive(Component)]
pub struct CountryFlagSlot;

#[derive(Component)]
pub struct CountryMetaText;

#[derive(Component)]
pub struct ProvinceMetaText;

#[derive(Component)]
pub struct AttackButton;

#[derive(Component)]
pub struct ShowProfileButton;

#[derive(Component)]
pub struct ProfileMetaText;

#[derive(Component)]
pub struct ChooseCountryButton;

#[derive(Component)]
pub struct BuildBankButton;

#[derive(Component)]
pub struct BuyArmyButton;

#[derive(Component)]
pub struct MakePeaceButton;

#[derive(Component)]
pub struct StartWarButton;

pub fn left_panel() -> impl Bundle {
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
        children![(
            Node {
                width: percent(100),
                flex_direction: FlexDirection::Row,
                column_gap: px(8),
                ..default()
            },
            children![
                action_button("Province", ShowProvinceButton),
                action_button("Country", ShowCountryButton),
                action_button("Profile", ShowProfileButton)
            ]
        ),],
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
    )
}

pub fn country_panel() -> impl Bundle {
    (
        Node {
            width: percent(100),
            flex_direction: FlexDirection::Column,
            row_gap: px(8),
            ..default()
        },
        children![
            country_flag_slot(),
            country_meta_text(),
            action_button("Choose Country", ChooseCountryButton),
            action_button("Make Peace", MakePeaceButton),
            action_button("Start War", StartWarButton),
        ],
    )
}

fn country_flag_slot() -> impl Bundle {
    (
        CountryFlagSlot,
        Node {
            width: px(96),
            height: px(64),
            border: UiRect::all(px(1)),
            margin: UiRect::bottom(px(10)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.04)),
        BorderColor::all(PANEL_BORDER),
        children![(
            ImageNode::default(),
            Node {
                width: percent(100.0),
                height: percent(100.0),
                ..default()
            },
        )],
    )
}

fn country_meta_text() -> impl Bundle {
    (
        Text::new("Country"),
        TextFont {
            font_size: 16.0,
            ..default()
        },
        InterfaceText { base_size: 16.0 },
        CountryMetaText,
    )
}

pub fn province_panel() -> impl Bundle {
    (
        Node {
            width: percent(100),
            flex_direction: FlexDirection::Column,
            row_gap: px(8),
            ..default()
        },
        children![
            province_meta_text(),
            action_button("Attack", AttackButton),
            action_button("Build Bank", BuildBankButton),
            action_button("Buy Army", BuyArmyButton)
        ],
    )
}

fn province_meta_text() -> impl Bundle {
    (
        Text::new("Province"),
        TextFont {
            font_size: 16.0,
            ..default()
        },
        InterfaceText { base_size: 16.0 },
        ProvinceMetaText,
    )
}

pub fn profile_meta_text() -> impl Bundle {
    (
        Text::new("Profile"),
        TextFont {
            font_size: 16.0,
            ..default()
        },
        InterfaceText { base_size: 16.0 },
        ProfileMetaText,
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
