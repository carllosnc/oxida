//! Themed Radio button component inspired by shadcn/ui.

use egui::{pos2, vec2, Response, Sense, Stroke, Ui, Widget, WidgetText};

use crate::theme;

/// A themed radio button bound to a selectable value, styled after shadcn/ui.
///
/// Features:
/// - Crisp 16px circular radio container
/// - Concentric dot indicator when selected
/// - Full interactive area covering both radio circle and label
///
/// ```
/// # use oxida::prelude::*;
/// # fn demo(ui: &mut egui::Ui, mut current: usize) {
/// ui.oxida().radio(&mut current, 1, "Opção 1").ui(ui);
/// ui.oxida().radio(&mut current, 2, "Opção 2").ui(ui);
/// # }
/// ```
pub struct Radio<'v, T: PartialEq> {
    current_value: &'v mut T,
    selected_value: T,
    label: Option<WidgetText>,
    enabled: bool,
}

impl<'v, T: PartialEq> Radio<'v, T> {
    /// Create a new radio button comparing `current_value` with `selected_value`.
    pub fn new(current_value: &'v mut T, selected_value: T) -> Self {
        Self {
            current_value,
            selected_value,
            label: None,
            enabled: true,
        }
    }

    /// Set an optional label shown to the right of the radio circle.
    pub fn label(mut self, label: impl Into<WidgetText>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Enable or disable the radio button.
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

impl<T: PartialEq + 'static> Widget for Radio<'_, T> {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = theme::current(ui.ctx());
        let p = &theme.palette;
        let m = &theme.metrics;

        let circle_diameter: f32 = 16.0;
        let circle_radius = circle_diameter / 2.0;
        let gap = m.spacing_unit * 2.0;

        let Radio {
            current_value,
            selected_value,
            label,
            enabled: widget_enabled,
        } = self;

        let is_enabled = ui.is_enabled() && widget_enabled;
        let is_selected = *current_value == selected_value;

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

        let height = circle_diameter.max(
            label_galley
                .as_ref()
                .map(|g| g.size().y)
                .unwrap_or(circle_diameter),
        );

        let desired = vec2(circle_diameter + label_w, height.max(20.0));
        let sense = if is_enabled {
            Sense::click()
        } else {
            Sense::hover()
        };
        let (rect, mut response) = ui.allocate_exact_size(desired, sense);

        let space_pressed = response.has_focus() && ui.input(|i| i.key_pressed(egui::Key::Space));
        if is_enabled && (response.clicked() || space_pressed) && !is_selected {
            *current_value = selected_value;
            response.mark_changed();
        }

        let center = pos2(rect.left() + circle_radius, rect.center().y);

        // Visuals (shadcn/ui style)
        let (circle_bg, border_stroke) = if !is_enabled {
            (
                p.subtle,
                Stroke::new(m.border_width, p.border),
            )
        } else if is_selected {
            (
                p.field_bg,
                Stroke::new(1.5, p.accent),
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

        // Outer circle
        ui.painter().circle(center, circle_radius, circle_bg, border_stroke);

        // Inner dot (bullet) when selected
        if is_selected {
            let dot_color = if !is_enabled {
                p.text_disabled
            } else {
                p.accent
            };
            let dot_radius = 3.5;
            ui.painter().circle_filled(center, dot_radius, dot_color);
        }

        // Label text on the right
        if let Some(g) = label_galley {
            let label_pos = pos2(
                rect.left() + circle_diameter + gap,
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
            ui.painter().circle_stroke(
                center,
                circle_radius + m.focus_ring_offset,
                Stroke::new(m.focus_ring_width, p.accent),
            );
        }

        response
    }
}
