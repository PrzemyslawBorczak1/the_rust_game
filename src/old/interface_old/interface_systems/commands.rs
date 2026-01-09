use bevy::prelude::*;

use crate::data::{GPUMaterial, GameWorld};

use super::super::common::*;

fn toggle_ui_visibility(
    mut vis: ParamSet<(
        Query<&mut Visibility, With<InterfaceRoot>>,
        Query<&mut Visibility, With<CountryFlagSlot>>,
    )>,
) {
    let mut now_hidden = false;
    {
        let mut root_q = vis.p0();
        if let Some(mut root_vis) = root_q.iter_mut().next() {
            *root_vis = match *root_vis {
                Visibility::Visible => Visibility::Hidden,
                Visibility::Hidden => Visibility::Visible,
                Visibility::Inherited => Visibility::Visible,
            };
            now_hidden = matches!(*root_vis, Visibility::Hidden);
        }
    }

    if now_hidden {
        let mut flag_q = vis.p1();
        if let Some(mut flag_vis) = flag_q.iter_mut().next() {
            *flag_vis = Visibility::Hidden;
        }
    }
}

pub fn apply_toggle_ui_event(
    mut events: MessageReader<ToggleUiEvent>,
    vis: ParamSet<(
        Query<&mut Visibility, With<InterfaceRoot>>,
        Query<&mut Visibility, With<CountryFlagSlot>>,
    )>,
) {
    let mut count = 0u32;
    for _ in events.read() {
        count += 1;
    }

    if count % 2 == 0 {
        return;
    }

    toggle_ui_visibility(vis);
}

pub fn apply_left_panel_view_event(
    mut events: MessageReader<SetLeftPanelViewEvent>,
    mut next_left_panel_view: ResMut<NextState<LeftPanelView>>,
) {
    let mut last = None;
    for ev in events.read() {
        last = Some(ev.0);
    }

    if let Some(view) = last {
        next_left_panel_view.set(view);
    }
}

pub fn apply_map_draw_mode_event(
    mut events: MessageReader<SetMapDrawModeEvent>,
    mut map_draw_mode: ResMut<MapDrawMode>,
    world: Res<GameWorld>,
    mut materials: ResMut<Assets<GPUMaterial>>,
    mut map_mode_label: Query<&mut Text, With<MapModeLabelText>>,
) {
    let mut last = None;
    for ev in events.read() {
        last = Some(ev.0);
    }

    let Some(mode) = last else {
        return;
    };

    *map_draw_mode = mode;
    set_map_draw_mode(&world, &mut materials, mode);

    let label = match mode {
        MapDrawMode::Political => "Selected: Political",
        MapDrawMode::Geographical => "Selected: Geographical",
    };

    if let Some(mut text) = map_mode_label.iter_mut().next() {
        *text = Text::new(label);
    }
}
