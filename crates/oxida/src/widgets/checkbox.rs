//! Themed Checkbox component inspired by shadcn/ui.

use egui::{pos2, vec2, Rect, Response, Sense, Stroke, StrokeKind, Ui, Widget, WidgetText};

use crate::theme;
use crate::util::radius;

/// A themed checkbox with an optional label, styled after shadcn/ui.
///
/// Features:
/// - Crisp 16x16 box with subtle rounding (4px)
/// - Clean vector checkmark indicator when checked
/// - Full interactive area covering both box and label
///
/// ```
/// # use oxida::prelude::*;
/// # fn demo(ui: &mut egui::Ui, mut agreed: bool) {
/// ui.oxida().checkbox(&mut agreed, "Aceito os termos").ui(ui);
/// # }
/// ```
pub struct Checkbox<'v> {
    checked: &'v mut bool,
    label: Option<WidgetText>,
    enabled: bool,
}

impl<'v> Checkbox<'v> {
    /// Create a checkbox bound to `checked`.
    pub fn new(checked: &'v mut bool) -> Self {
        Self {
            checked,
            label: None,
            enabled: true,
        }
    }

    /// Set an optional label shown to the right of the checkbox.
    pub fn label(mut self, label: impl Into<WidgetText>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Enable or disable the checkbox.
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

impl Widget for Checkbox<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = theme::current(ui.ctx());
        let p = &theme.palette;
        let m = &theme.metrics;

        let box_size: f32 = 16.0;
        let gap = m.spacing_unit * 2.0;

        let Checkbox {
            checked,
            label,
            enabled: widget_enabled,
        } = self;

        let is_enabled = ui.is_enabled() && widget_enabled;

        let font_id = theme.typography.font_id(crate::widgets::Size::Md);
        let label_galley = label.map(|l| {
            l.into_galley(
                ui,
                Some(egui::TextWrapMode::Extend),
                f32::INFINITY,
                font_id,
            )
        });

        let label_w = label_galley
            .as_ref()
            .map(|g| g.size().x + gap)
            .unwrap_or(0.0);

        let height = box_size.max(
            label_galley
                .as_ref()
                .map(|g| g.size().y)
                .unwrap_or(box_size),
        );

        let desired = vec2(box_size + label_w, height.max(20.0));
        let sense = if is_enabled {
            Sense::click()
        } else {
            Sense::hover()
        };
        let (rect, mut response) = ui.allocate_exact_size(desired, sense);

        let space_pressed = response.has_focus() && ui.input(|i| i.key_pressed(egui::Key::Space));
        if is_enabled && (response.clicked() || space_pressed) {
            *checked = !*checked;
            response.mark_changed();
        }

        let is_checked = *checked;
        let t = ui
            .ctx()
            .animate_bool_with_time(response.id, is_checked, 0.12);

        // Box rect on the left
        let box_rect = Rect::from_center_size(
            pos2(rect.left() + box_size / 2.0, rect.center().y),
            vec2(box_size, box_size),
        );

        let box_rounding = radius(m.radius_sm);

        // Visuals (shadcn/ui style)
        let (bg, border_stroke) = if !is_enabled {
            (
                p.subtle,
                Stroke::new(m.border_width, p.border),
            )
        } else if is_checked {
            (
                p.accent,
                Stroke::new(m.border_width, p.accent),
            )
        } else if response.hovered() {
            (
                p.field_bg,
                Stroke::new(m.border_width, p.border_strong),
            )
        } else {
            (
                p.field_bg,
                Stroke::new(m.border_width, p.border),
            )
        };

        ui.painter().rect_filled(box_rect, box_rounding, bg);
        ui.painter()
            .rect_stroke(box_rect, box_rounding, border_stroke, StrokeKind::Inside);

        // Paint checkmark when checked
        if is_checked || t > 0.05 {
            let check_color = if !is_enabled {
                p.text_disabled
            } else {
                p.on_accent
            };

            let stroke = Stroke::new(1.6, check_color);
            let cx = box_rect.center().x;
            let cy = box_rect.center().y;

            // Draw checkmark lines
            ui.painter().line_segment(
                [pos2(cx - 3.8, cy - 0.2), pos2(cx - 1.2, cy + 2.5)],
                stroke,
            );
            ui.painter().line_segment(
                [pos2(cx - 1.2, cy + 2.5), pos2(cx + 3.8, cy - 2.8)],
                stroke,
            );
        }

        // Paint label
        if let Some(g) = label_galley {
            let label_pos = pos2(
                box_rect.right() + gap,
                rect.center().y - g.size().y / 2.0,
            );
            let text_color = if !is_enabled {
                p.text_disabled
            } else {
                p.text_primary
            };
            ui.painter().galley(label_pos, g, text_color);
        }

        // Focus ring (WCAG 2.4.7 / 2.4.11)
        if response.has_focus() {
            ui.painter().rect_stroke(
                box_rect.expand(m.focus_ring_offset),
                radius(m.radius_sm + m.focus_ring_offset),
                Stroke::new(m.focus_ring_width, p.accent),
                StrokeKind::Outside,
            );
        }

        response
    }
}
