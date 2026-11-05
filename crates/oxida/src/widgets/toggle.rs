//! Themed on/off switch component inspired by modern desktop design systems.

use egui::{pos2, vec2, Color32, Rect, Response, Sense, Stroke, StrokeKind, Ui, Widget, WidgetText};

use crate::theme;
use crate::util::{mix, radius};
use super::Size;

/// A themed on/off switch, optionally with a label.
///
/// Features:
/// - Desktop sizes: `Sm` (28 × 16 px track / 30 px row) and `Md` (32 × 18 px track / 38 px row)
/// - High-contrast visual thumb differentiating state without relying solely on color
/// - Full interactive area with keyboard spacebar toggling and focus ring
///
/// ```
/// # use oxida::prelude::*;
/// # fn demo(ui: &mut egui::Ui, mut enabled: bool) {
/// ui.oxida()
///     .toggle(&mut enabled)
///     .label("Notifications")
///     .ui(ui);
/// # }
/// ```
pub struct Toggle<'v> {
    value: &'v mut bool,
    label: Option<WidgetText>,
    size: Size,
    enabled: bool,
}

impl<'v> Toggle<'v> {
    /// Create a toggle bound to `value`.
    pub fn new(value: &'v mut bool) -> Self {
        Self {
            value,
            label: None,
            size: Size::Md,
            enabled: true,
        }
    }

    /// Set the size of the switch (`Size::Sm` or `Size::Md`).
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    /// Label shown to the right of the switch.
    pub fn label(mut self, label: impl Into<WidgetText>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Enable or disable the toggle.
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

impl Widget for Toggle<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = theme::current(ui.ctx());
        let p = &theme.palette;
        let m = &theme.metrics;

        let (track_w, track_h, knob_r) = match self.size {
            Size::Sm => (28.0, 16.0, 5.5),
            Size::Md | Size::Lg => (32.0, 18.0, 6.5),
        };
        let row_height = match self.size {
            Size::Sm => m.control_height_sm,
            Size::Md | Size::Lg => m.control_height_md,
        };
        let pad = (track_h - 2.0 * knob_r) / 2.0;
        let gap = 8.0;

        let Toggle {
            value,
            label,
            size,
            enabled: widget_enabled,
        } = self;

        let is_enabled = ui.is_enabled() && widget_enabled;

        let label_color = if is_enabled {
            p.text_primary
        } else {
            p.text_disabled
        };

        let font_id = theme.typography.font_id(size);
        let label_galley = label.map(|l| {
            ui.scope(|ui| {
                ui.style_mut().visuals.override_text_color = Some(label_color);
                l.into_galley(
                    ui,
                    Some(egui::TextWrapMode::Extend),
                    f32::INFINITY,
                    font_id,
                )
            })
            .inner
        });

        let label_w = label_galley
            .as_ref()
            .map(|g| g.size().x + gap)
            .unwrap_or(0.0);

        let sense = if is_enabled {
            Sense::click()
        } else {
            Sense::hover()
        };
        let (rect, mut response) = ui.allocate_exact_size(vec2(track_w + label_w, row_height), sense);

        let switch_rect = Rect::from_min_size(
            pos2(rect.left(), rect.center().y - track_h / 2.0),
            vec2(track_w, track_h),
        );

        let space_pressed = response.has_focus() && ui.input(|i| i.key_pressed(egui::Key::Space));
        if is_enabled && (response.clicked() || space_pressed) {
            *value = !*value;
            response.mark_changed();
        }

        // Paint
        let painter = ui.painter();
        let t = ui.ctx().animate_bool_with_time(response.id, *value, 0.14);
        let hover_t = ui
            .ctx()
            .animate_bool_with_time(response.id.with("hover"), is_enabled && response.hovered(), 0.10);

        let (track_color, track_border, knob_color) = if is_enabled {
            if theme.dark_mode {
                let off_track = p.surface;
                let off_border = mix(p.border, Color32::from_rgb(0x71, 0x71, 0x7A), hover_t);
                let off_knob = Color32::from_rgb(0xF5, 0xF5, 0xF7);

                let on_track = Color32::from_rgb(0xF5, 0xF5, 0xF7);
                let on_border = Color32::TRANSPARENT;
                let on_knob = Color32::from_rgb(0x14, 0x14, 0x16);

                (
                    mix(off_track, on_track, t),
                    mix(off_border, on_border, t),
                    mix(off_knob, on_knob, t),
                )
            } else {
                let off_track = p.subtle;
                let off_border = mix(p.border, p.border_strong, hover_t);
                let off_knob = Color32::WHITE;

                let on_track = p.accent;
                let on_border = Color32::TRANSPARENT;
                let on_knob = p.on_accent;

                (
                    mix(off_track, on_track, t),
                    mix(off_border, on_border, t),
                    mix(off_knob, on_knob, t),
                )
            }
        } else {
            let dis_track = if *value { p.subtle } else { p.surface };
            let dis_border = p.border_subtle;
            let dis_knob = p.text_disabled;
            (dis_track, dis_border, dis_knob)
        };

        // Draw track
        let track_rounding = radius(track_h / 2.0);
        painter.rect_filled(switch_rect, track_rounding, track_color);
        if track_border != Color32::TRANSPARENT {
            painter.rect_stroke(
                switch_rect,
                track_rounding,
                Stroke::new(1.0, track_border),
                StrokeKind::Inside,
            );
        }

        // Draw knob
        let travel = track_w - 2.0 * pad - 2.0 * knob_r;
        let knob_x = switch_rect.left() + pad + knob_r + t * travel;
        painter.circle_filled(pos2(knob_x, rect.center().y), knob_r, knob_color);

        // Draw label to the right of the switch
        if let Some(ref g) = label_galley {
            let pos = pos2(switch_rect.right() + gap, rect.center().y - g.size().y / 2.0);
            painter.galley(pos, g.clone(), label_color);
        }

        // Focus ring (WCAG 2.4.7 / 2.4.11 / Figma focus/ring #B6A4FF)
        if response.has_focus() {
            painter.rect_stroke(
                switch_rect.expand(m.focus_ring_offset),
                radius(track_h / 2.0 + m.focus_ring_offset),
                Stroke::new(m.focus_ring_width, p.focus_ring),
                StrokeKind::Outside,
            );
        }

        response
    }
}
