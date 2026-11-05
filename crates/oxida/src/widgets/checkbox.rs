//! Themed Checkbox component inspired by modern desktop design systems.

use egui::{pos2, vec2, Color32, Rect, Response, Sense, Stroke, StrokeKind, Ui, Widget, WidgetText};

use crate::theme;
use crate::util::radius;
use super::Size;

/// Tri-state value for checkboxes supporting indeterminate selection.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CheckboxState {
    /// Desmarcado / Unchecked.
    #[default]
    Unchecked,
    /// Marcado / Checked.
    Checked,
    /// Misto / Indeterminate (partially selected group).
    Mixed,
}

enum CheckboxBinding<'v> {
    Bool(&'v mut bool, bool), // (&mut bool, is_indeterminate)
    State(&'v mut CheckboxState),
}

/// A themed checkbox with an optional label, styled after contemporary desktop HIG.
///
/// Features:
/// - Desktop sizes: `Sm` (16 px box / 30 px row) and `Md` (18 px box / 38 px row)
/// - Vector checkmark and indeterminate horizontal minus indicators
/// - Full interactive area covering both box and label with keyboard focus ring
///
/// ```
/// # use oxida::prelude::*;
/// # fn demo(ui: &mut egui::Ui, mut agreed: bool) {
/// ui.oxida().checkbox(&mut agreed, "Aceito os termos").ui(ui);
/// # }
/// ```
pub struct Checkbox<'v> {
    binding: CheckboxBinding<'v>,
    label: Option<WidgetText>,
    size: Size,
    enabled: bool,
}

impl<'v> Checkbox<'v> {
    /// Create a checkbox bound to a boolean `checked`.
    pub fn new(checked: &'v mut bool) -> Self {
        Self {
            binding: CheckboxBinding::Bool(checked, false),
            label: None,
            size: Size::Md,
            enabled: true,
        }
    }

    /// Create a checkbox bound to a tri-state [`CheckboxState`].
    pub fn state(state: &'v mut CheckboxState) -> Self {
        Self {
            binding: CheckboxBinding::State(state),
            label: None,
            size: Size::Md,
            enabled: true,
        }
    }

    /// Mark the boolean-bound checkbox as indeterminate (`Mixed`).
    pub fn indeterminate(mut self, indeterminate: bool) -> Self {
        if let CheckboxBinding::Bool(ref mut _b, ref mut ind) = self.binding {
            *ind = indeterminate;
        }
        self
    }

    /// Set the size of the checkbox control (`Size::Sm` or `Size::Md`).
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
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
    fn ui(mut self, ui: &mut Ui) -> Response {
        let theme = theme::current(ui.ctx());
        let p = &theme.palette;
        let m = &theme.metrics;

        let box_size: f32 = match self.size {
            Size::Sm => 16.0,
            Size::Md | Size::Lg => 18.0,
        };
        let row_height: f32 = match self.size {
            Size::Sm => m.control_height_sm,
            Size::Md | Size::Lg => m.control_height_md,
        };
        let gap = 8.0;

        let current_state = match self.binding {
            CheckboxBinding::Bool(ref b, ind) => {
                if ind {
                    CheckboxState::Mixed
                } else if **b {
                    CheckboxState::Checked
                } else {
                    CheckboxState::Unchecked
                }
            }
            CheckboxBinding::State(ref s) => **s,
        };

        let is_enabled = ui.is_enabled() && self.enabled;

        let label_color = if is_enabled {
            p.text_primary
        } else {
            p.text_disabled
        };

        let font_id = theme.typography.font_id(self.size);
        let label_galley = self.label.map(|l| {
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

        let desired = vec2(box_size + label_w, row_height);
        let sense = if is_enabled {
            Sense::click()
        } else {
            Sense::hover()
        };
        let (rect, mut response) = ui.allocate_exact_size(desired, sense);

        let space_pressed = response.has_focus() && ui.input(|i| i.key_pressed(egui::Key::Space));
        if is_enabled && (response.clicked() || space_pressed) {
            match self.binding {
                CheckboxBinding::Bool(ref mut b, ref mut ind) => {
                    if *ind {
                        *ind = false;
                        **b = true;
                    } else {
                        **b = !**b;
                    }
                }
                CheckboxBinding::State(ref mut s) => {
                    **s = match **s {
                        CheckboxState::Unchecked => CheckboxState::Checked,
                        CheckboxState::Checked => CheckboxState::Unchecked,
                        CheckboxState::Mixed => CheckboxState::Checked,
                    };
                }
            }
            response.mark_changed();
        }

        let is_checked_or_mixed = current_state != CheckboxState::Unchecked;

        // Visual animations
        let check_t = ui
            .ctx()
            .animate_bool_with_time(response.id, is_checked_or_mixed, 0.12);
        let hover_t = ui
            .ctx()
            .animate_bool_with_time(response.id.with("hover"), is_enabled && response.hovered(), 0.10);

        let box_rect = Rect::from_min_size(
            pos2(rect.left(), rect.center().y - box_size / 2.0),
            vec2(box_size, box_size),
        );

        let (bg, border_stroke, icon_color) = if is_enabled {
            if theme.dark_mode {
                let uncheck_border = crate::util::mix(p.border, Color32::from_rgb(0x71, 0x71, 0x7A), hover_t);
                let uncheck_bg = if response.is_pointer_button_down_on() {
                    p.subtle
                } else {
                    p.surface
                };
                let check_bg = Color32::from_rgb(0xF5, 0xF5, 0xF7);
                let bg = crate::util::mix(uncheck_bg, check_bg, check_t);
                let stroke = Stroke::new(
                    1.0,
                    crate::util::mix(uncheck_border, Color32::TRANSPARENT, check_t),
                );
                let icon_c = Color32::from_rgb(0x14, 0x14, 0x16);
                (bg, stroke, icon_c)
            } else {
                let uncheck_border = crate::util::mix(p.border, p.border_strong, hover_t);
                let uncheck_bg = if response.is_pointer_button_down_on() {
                    p.subtle
                } else {
                    p.surface
                };
                let check_bg = p.accent;
                let bg = crate::util::mix(uncheck_bg, check_bg, check_t);
                let stroke = Stroke::new(
                    1.0,
                    crate::util::mix(uncheck_border, Color32::TRANSPARENT, check_t),
                );
                let icon_c = p.on_accent;
                (bg, stroke, icon_c)
            }
        } else {
            let dis_bg = if is_checked_or_mixed {
                p.subtle
            } else {
                p.surface
            };
            let dis_border = Stroke::new(1.0, p.border_subtle);
            let dis_icon = p.text_disabled;
            (dis_bg, dis_border, dis_icon)
        };

        let painter = ui.painter();
        let box_radius = radius(4.5);

        // Box background & border
        painter.rect_filled(box_rect, box_radius, bg);
        if border_stroke.color != Color32::TRANSPARENT {
            painter.rect_stroke(box_rect, box_radius, border_stroke, StrokeKind::Inside);
        }

        // Draw indicator: checkmark or mixed horizontal line
        if check_t > 0.05 {
            let ind_color = Color32::from_rgba_premultiplied(
                (f32::from(icon_color.r()) * check_t).round() as u8,
                (f32::from(icon_color.g()) * check_t).round() as u8,
                (f32::from(icon_color.b()) * check_t).round() as u8,
                (f32::from(icon_color.a()) * check_t).round() as u8,
            );

            if current_state == CheckboxState::Mixed {
                // Horizontal minus dash
                let half_bar = box_size * 0.26;
                let y = box_rect.center().y;
                painter.line_segment(
                    [
                        pos2(box_rect.center().x - half_bar, y),
                        pos2(box_rect.center().x + half_bar, y),
                    ],
                    Stroke::new(2.0, ind_color),
                );
            } else {
                // Vector checkmark
                let stroke_w = if box_size >= 18.0 { 2.0 } else { 1.75 };
                let p1 = pos2(box_rect.left() + box_size * 0.24, box_rect.top() + box_size * 0.50);
                let p2 = pos2(box_rect.left() + box_size * 0.44, box_rect.top() + box_size * 0.72);
                let p3 = pos2(box_rect.left() + box_size * 0.76, box_rect.top() + box_size * 0.28);

                painter.line_segment([p1, p2], Stroke::new(stroke_w, ind_color));
                painter.line_segment([p2, p3], Stroke::new(stroke_w, ind_color));
            }
        }

        // Label
        if let Some(ref galley) = label_galley {
            let label_pos = pos2(
                box_rect.right() + gap,
                rect.center().y - galley.size().y / 2.0,
            );
            painter.galley(label_pos, galley.clone(), label_color);
        }

        // Focus ring (WCAG 2.4.7 / 2.4.11 / Figma focus/ring #B6A4FF)
        if response.has_focus() {
            painter.rect_stroke(
                box_rect.expand(m.focus_ring_offset),
                radius(4.5 + m.focus_ring_offset),
                Stroke::new(m.focus_ring_width, p.focus_ring),
                StrokeKind::Outside,
            );
        }

        response
    }
}
