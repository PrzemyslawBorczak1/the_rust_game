use bevy::prelude::*;
use bevy::window::WindowResized;

use super::super::common::*;

pub fn interface_apply_responsive_layout(
    mut resized: MessageReader<WindowResized>,
    window: Single<&Window>,
    mut nodes: ParamSet<(
        Query<&mut Node, With<InterfaceLeftPanel>>,
        Query<&mut Node, With<InterfaceRightPanel>>,
        Query<&mut Node, With<LeftPanelBody>>,
        Query<&mut Node, With<CountryFlagSlot>>,
    )>,
    mut texts: Query<(&InterfaceText, &mut TextFont)>,
    mut did_init: Local<bool>,
) {
    let resized_now = resized.read().next().is_some();
    if !resized_now && *did_init {
        return;
    }
    *did_init = true;

    let w = window.width();
    let h = window.height();
    let scale = (h / 900.0).clamp(0.9, 1.5);

    let panel_w = (w * 0.23).clamp(280.0, 520.0);
    let panel_pad = (10.0 * scale).clamp(8.0, 18.0);
    let inner_pad = (8.0 * scale).clamp(6.0, 14.0);

    {
        let mut q = nodes.p0();
        if let Some(mut node) = q.iter_mut().next() {
            node.width = Val::Px(panel_w);
            node.padding = UiRect::all(Val::Px(panel_pad));
        }
    }
    {
        let mut q = nodes.p1();
        if let Some(mut node) = q.iter_mut().next() {
            node.width = Val::Px(panel_w);
            node.padding = UiRect::all(Val::Px(panel_pad));
        }
    }
    {
        let mut q = nodes.p2();
        if let Some(mut node) = q.iter_mut().next() {
            node.padding = UiRect::all(Val::Px(inner_pad));
        }
    }
    {
        let mut q = nodes.p3();
        if let Some(mut node) = q.iter_mut().next() {
            node.width = Val::Px((96.0 * scale).round());
            node.height = Val::Px((64.0 * scale).round());
            node.margin = UiRect::bottom(Val::Px((10.0 * scale).round()));
        }
    }

    for (cfg, mut font) in &mut texts {
        font.font_size = cfg.base_size * scale;
    }
}
