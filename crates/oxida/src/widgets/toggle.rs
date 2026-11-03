//! Themed on/off switch.

use egui::{pos2, vec2, Rect, Response, Sense, Stroke, StrokeKind, Ui, Widget, WidgetText};

use crate::theme;
use crate::util::{mix, radius};
use super::Size;

/// A themed on/off switch, optionally with a label.
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
}

impl<'v> Toggle<'v> {
    /// Create a toggle bound to `value`.
    pub fn new(value: &'v mut bool) -> Self {
        Self { value, label: None }
    }

    /// Label shown to the left of the switch.
    pub fn label(mut self, label: impl Into<WidgetText>) -> Self {
        self.label = Some(label.into());
        self
    }
}

impl Widget for Toggle<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = theme::current(ui.ctx());
        let p = &theme.palette;
        let m = &theme.metrics;

        let track_w = 34.0;
        let track_h = 20.0;
        let knob_r = 7.0;
        let pad = (track_h - 2.0 * knob_r) / 2.0;
        let gap = m.spacing_unit * 2.5;

        let Toggle { value, label } = self;
        let label_galley = label.map(|l| {
            l.into_galley(
                ui,
                Some(egui::TextWrapMode::Extend),
                f32::INFINITY,
                theme.typography.font_id(Size::Md),
            )
        });
        let label_w = label_galley
            .as_ref()
            .map(|g| g.size().x + gap)
            .unwrap_or(0.0);

        let enabled = ui.is_enabled();
        let sense = if enabled { Sense::click() } else { Sense::hover() };
        let (rect, mut response) = ui.allocate_exact_size(vec2(label_w + track_w, track_h), sense);

        let switch_rect = Rect::from_min_size(
            pos2(rect.left() + label_w, rect.center().y - track_h / 2.0),
            vec2(track_w, track_h),
        );

        let space_pressed = response.has_focus() && ui.input(|i| i.key_pressed(egui::Key::Space));
        if enabled && (response.clicked() || space_pressed) {
            *value = !*value;
            response.mark_changed();
        }

        // Paint.
        let painter = ui.painter_at(rect);
        let t = ui.ctx().animate_bool_with_time(response.id, *value, 0.15);

        let on = p.accent;
        let off = p.border_strong;
        let (track_color, knob_color) = if enabled {
            (mix(off, on, t), mix(p.surface, p.on_accent, t))
        } else {
            (
                crate::util::dim(mix(off, on, t), p.window_bg, 0.55),
                crate::util::dim(p.surface, p.window_bg, 0.55),
            )
        };

        painter.rect_filled(
            switch_rect,
            radius(track_h / 2.0),
            track_color,
        );

        let travel = track_w - 2.0 * pad - 2.0 * knob_r;
        let knob_x = switch_rect.left() + pad + knob_r + t * travel;
        painter.circle_filled(pos2(knob_x, rect.center().y), knob_r, knob_color);

        if let Some(g) = label_galley {
            let pos = pos2(rect.left(), rect.center().y - g.size().y / 2.0);
            let color = if enabled { p.text_primary } else { p.text_disabled };
            painter.galley(pos, g, color);
        }

        // Focus ring (WCAG 2.4.7 / 2.4.11)
        if response.has_focus() {
            painter.rect_stroke(
                switch_rect.expand(m.focus_ring_offset),
                radius(track_h / 2.0 + m.focus_ring_offset),
                Stroke::new(m.focus_ring_width, p.accent),
                StrokeKind::Outside,
            );
        }

        response
    }
}
