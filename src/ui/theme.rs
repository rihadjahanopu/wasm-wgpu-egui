use egui::{Color32, Margin, Rounding, Stroke, Visuals};

/// Configures an ultra-clean, professional dark IDE/engine studio styling.
pub fn apply_studio_theme(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();

    // Dark studio palette (inspired by Godot & Blender)
    let bg_dark = Color32::from_rgb(17, 20, 26);
    let panel_bg = Color32::from_rgb(23, 27, 34);
    let element_bg = Color32::from_rgb(32, 38, 48);
    let accent_blue = Color32::from_rgb(66, 133, 244);
    let text_primary = Color32::from_rgb(235, 240, 245);
    let text_muted = Color32::from_rgb(145, 155, 170);

    let mut visuals = Visuals::dark();
    visuals.override_text_color = Some(text_primary);
    visuals.panel_fill = panel_bg;
    visuals.window_fill = panel_bg;
    visuals.extreme_bg_color = bg_dark;

    // Window & Widget styling
    visuals.window_stroke = Stroke::new(1.0_f32, Color32::from_rgb(45, 52, 65));
    visuals.window_rounding = Rounding::same(6.0);

    visuals.widgets.noninteractive.bg_fill = panel_bg;
    visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0_f32, text_muted);
    visuals.widgets.noninteractive.rounding = Rounding::same(4.0);

    visuals.widgets.inactive.bg_fill = element_bg;
    visuals.widgets.inactive.fg_stroke = Stroke::new(1.0_f32, text_primary);
    visuals.widgets.inactive.rounding = Rounding::same(4.0);

    visuals.widgets.hovered.bg_fill = Color32::from_rgb(42, 50, 64);
    visuals.widgets.hovered.fg_stroke = Stroke::new(1.0_f32, Color32::WHITE);
    visuals.widgets.hovered.rounding = Rounding::same(4.0);

    visuals.widgets.active.bg_fill = accent_blue;
    visuals.widgets.active.fg_stroke = Stroke::new(1.0_f32, Color32::WHITE);
    visuals.widgets.active.rounding = Rounding::same(4.0);

    visuals.selection.bg_fill = accent_blue;

    style.visuals = visuals;
    style.spacing.item_spacing = egui::vec2(8.0, 6.0);
    style.spacing.window_margin = Margin::same(10.0);
    style.spacing.button_padding = egui::vec2(8.0, 4.0);

    ctx.set_style(style);
}
