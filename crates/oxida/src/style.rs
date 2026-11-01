//! Maps Oxida design tokens onto [`egui::Style`], so that egui's built-in
//! widgets (panels, menus, `TextEdit`, `ComboBox`, ...) match the theme too.

use egui::style::WidgetVisuals;
use egui::{CornerRadius, Margin, Stroke, Style, Vec2, Visuals};

use crate::Theme;

pub(crate) fn to_egui_style(theme: &Theme) -> Style {
    let p = &theme.palette;
    let m = &theme.metrics;
    let t = &theme.typography;

    let mut style = Style::default();

    // Typography ----------------------------------------------------------
    for (style_text, font_id) in t.text_styles() {
        style.text_styles.insert(style_text, font_id);
    }

    // Spacing ---------------------------------------------------------------
    let unit = m.spacing_unit;
    style.spacing.item_spacing = Vec2::new(unit * 2.0, unit * 1.5);
    style.spacing.button_padding = Vec2::new(unit * 2.5, unit);
    style.spacing.interact_size = Vec2::new(unit * 6.0, m.control_height_md);
    style.spacing.menu_margin = Margin::same((unit * 1.5) as i8);

    // Visuals ----------------------------------------------------------------
    let mut visuals = if theme.dark_mode {
        Visuals::dark()
    } else {
        Visuals::light()
    };

    visuals.dark_mode = theme.dark_mode;

    // Text rasterization: hinting keeps baselines/x-height on the pixel grid,
    // and sub-pixel binning keeps each glyph at its true fractional position.
    // Turning binning off snaps every glyph to whole pixels, which produces
    // uneven letter spacing and a jagged, "pixelated" look.
    visuals.text_options.font_hinting = true;
    visuals.text_options.subpixel_binning = true;

    visuals.override_text_color = Some(p.text_primary);
    visuals.window_fill = p.surface;
    visuals.panel_fill = if theme.dark_mode { p.window_bg } else { p.surface };
    visuals.extreme_bg_color = p.field_bg;
    visuals.faint_bg_color = p.subtle;
    visuals.selection.bg_fill = p.accent;
    visuals.selection.stroke = Stroke::new(1.0, p.on_accent);

    let control_rounding = CornerRadius::same(m.radius_md as u8);
    let surface_rounding = CornerRadius::same(m.radius_lg as u8);

    visuals.widgets.inactive = WidgetVisuals {
        bg_fill: p.surface,
        weak_bg_fill: p.surface,
        bg_stroke: Stroke::new(m.border_width, p.border),
        corner_radius: control_rounding,
        fg_stroke: Stroke::new(1.0, p.text_primary),
        expansion: 0.0,
    };
    visuals.widgets.hovered = WidgetVisuals {
        bg_fill: p.subtle,
        weak_bg_fill: p.subtle,
        bg_stroke: Stroke::new(m.border_width, p.border_strong),
        corner_radius: control_rounding,
        fg_stroke: Stroke::new(1.0, p.text_primary),
        expansion: 0.0,
    };
    visuals.widgets.active = WidgetVisuals {
        bg_fill: p.border,
        weak_bg_fill: p.border,
        bg_stroke: Stroke::new(m.border_width, p.border_strong),
        corner_radius: control_rounding,
        fg_stroke: Stroke::new(1.0, p.text_primary),
        expansion: 0.0,
    };
    visuals.widgets.open = visuals.widgets.hovered;
    visuals.widgets.noninteractive = WidgetVisuals {
        bg_fill: p.surface,
        weak_bg_fill: p.subtle,
        bg_stroke: Stroke::new(m.border_width, p.border),
        corner_radius: control_rounding,
        fg_stroke: Stroke::new(1.0, p.text_secondary),
        expansion: 0.0,
    };

    visuals.window_corner_radius = surface_rounding;
    visuals.menu_corner_radius = control_rounding;
    visuals.window_stroke = Stroke::new(m.border_width, p.border);
    visuals.hyperlink_color = p.accent;
    visuals.warn_fg_color = p.warning;
    visuals.error_fg_color = p.danger;

    let mut popup_shadow = egui::Shadow::default();
    popup_shadow.color = if theme.dark_mode {
        egui::Color32::from_black_alpha(120)
    } else {
        egui::Color32::from_black_alpha(32)
    };
    visuals.popup_shadow = popup_shadow;

    style.visuals = visuals;
    style
}
