//! Themed Radio button component inspired by modern desktop design systems.

use egui::{pos2, vec2, Color32, Response, Sense, Stroke, Ui, Widget, WidgetText};

use crate::theme;
use super::Size;

/// A themed radio button bound to a selectable value, styled after contemporary desktop HIG.
///
/// Features:
/// - Desktop sizes: `Sm` (16 px circle / 30 px row) and `Md` (18 px circle / 38 px row)
/// - Concentric dot indicator when selected
/// - Full interactive area covering both radio circle and label with keyboard focus ring
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
    size: Size,
    enabled: bool,
}

impl<'v, T: PartialEq> Radio<'v, T> {
    /// Create a new radio button comparing `current_value` with `selected_value`.
    pub fn new(current_value: &'v mut T, selected_value: T) -> Self {
        Self {
            current_value,
            selected_value,
            label: None,
            size: Size::Md,
            enabled: true,
        }
    }

    /// Set the size of the radio control (`Size::Sm` or `Size::Md`).
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
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

        let circle_diameter: f32 = match self.size {
            Size::Sm => 16.0,
            Size::Md | Size::Lg => 18.0,
        };
        let circle_radius = circle_diameter / 2.0;
        let row_height = match self.size {
            Size::Sm => m.control_height_sm,
            Size::Md | Size::Lg => m.control_height_md,
        };
        let gap = 8.0;

        let Radio {
            current_value,
            selected_value,
            label,
            size,
            enabled: widget_enabled,
        } = self;

        let is_enabled = ui.is_enabled() && widget_enabled;
        let is_selected = *current_value == selected_value;

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

        let desired = vec2(circle_diameter + label_w, row_height);
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

        let select_t = ui
            .ctx()
            .animate_bool_with_time(response.id, is_selected, 0.12);
        let hover_t = ui
            .ctx()
            .animate_bool_with_time(response.id.with("hover"), is_enabled && response.hovered(), 0.10);

        let (circle_bg, border_stroke, dot_color) = if is_enabled {
            if theme.dark_mode {
                let unselect_border = crate::util::mix(p.border, Color32::from_rgb(0x71, 0x71, 0x7A), hover_t);
                let unselect_bg = if response.is_pointer_button_down_on() {
                    p.subtle
                } else {
                    p.surface
                };
                let select_bg = Color32::from_rgb(0xF5, 0xF5, 0xF7);
                let bg = crate::util::mix(unselect_bg, select_bg, select_t);
                let stroke = Stroke::new(
                    1.0,
                    crate::util::mix(unselect_border, Color32::TRANSPARENT, select_t),
                );
                let dot_c = Color32::from_rgb(0x14, 0x14, 0x16);
                (bg, stroke, dot_c)
            } else {
                let unselect_border = crate::util::mix(p.border, p.border_strong, hover_t);
                let unselect_bg = if response.is_pointer_button_down_on() {
                    p.subtle
                } else {
                    p.surface
                };
                let select_bg = p.accent;
                let bg = crate::util::mix(unselect_bg, select_bg, select_t);
                let stroke = Stroke::new(
                    1.0,
                    crate::util::mix(unselect_border, Color32::TRANSPARENT, select_t),
                );
                let dot_c = p.on_accent;
                (bg, stroke, dot_c)
            }
        } else {
            let dis_bg = if is_selected {
                p.subtle
            } else {
                p.surface
            };
            let dis_border = Stroke::new(1.0, p.border_subtle);
            let dis_dot = p.text_disabled;
            (dis_bg, dis_border, dis_dot)
        };

        // Outer circle
        ui.painter().circle(center, circle_radius, circle_bg, border_stroke);

        // Inner dot (concentric bullet) when selected
        if select_t > 0.05 {
            let dot_radius = (circle_radius * 0.44) * select_t;
            let current_dot_color = Color32::from_rgba_premultiplied(
                (f32::from(dot_color.r()) * select_t).round() as u8,
                (f32::from(dot_color.g()) * select_t).round() as u8,
                (f32::from(dot_color.b()) * select_t).round() as u8,
                (f32::from(dot_color.a()) * select_t).round() as u8,
            );
            ui.painter().circle_filled(center, dot_radius, current_dot_color);
        }

        // Label
        if let Some(ref galley) = label_galley {
            let label_pos = pos2(
                rect.left() + circle_diameter + gap,
                rect.center().y - galley.size().y / 2.0,
            );
            ui.painter().galley(label_pos, galley.clone(), label_color);
        }

        // Focus ring (WCAG 2.4.7 / 2.4.11 / Figma focus/ring #B6A4FF)
        if response.has_focus() {
            ui.painter().circle_stroke(
                center,
                circle_radius + m.focus_ring_offset,
                Stroke::new(m.focus_ring_width, p.focus_ring),
            );
        }

        response
    }
}
