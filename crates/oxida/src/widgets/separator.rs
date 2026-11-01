//! Themed Separator / Divider widget.

use egui::{pos2, vec2, FontId, Response, Sense, Stroke, Ui, Widget, WidgetText};

use crate::theme;

/// Direction of the separator.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SeparatorDirection {
    /// Horizontal divider (the default).
    #[default]
    Horizontal,
    /// Vertical divider.
    Vertical,
}

/// A hairline divider that uses the active theme border color.
///
/// Can be horizontal, vertical, or horizontal with a centered text label.
///
/// ```
/// # use oxida::prelude::*;
/// # fn demo(ui: &mut egui::Ui) {
/// Separator::horizontal().ui(ui);
/// Separator::with_label("OU").ui(ui);
/// # }
/// ```
#[derive(Clone)]
pub struct Separator {
    direction: SeparatorDirection,
    spacing: f32,
    label: Option<WidgetText>,
}

impl Default for Separator {
    fn default() -> Self {
        Self::horizontal()
    }
}

impl Separator {
    /// Create a horizontal separator with default spacing.
    pub fn horizontal() -> Self {
        Self {
            direction: SeparatorDirection::Horizontal,
            spacing: 16.0,
            label: None,
        }
    }

    /// Create a vertical separator with default spacing.
    pub fn vertical() -> Self {
        Self {
            direction: SeparatorDirection::Vertical,
            spacing: 8.0,
            label: None,
        }
    }

    /// Create a horizontal separator with a centered label.
    pub fn with_label(label: impl Into<WidgetText>) -> Self {
        Self {
            direction: SeparatorDirection::Horizontal,
            spacing: 20.0,
            label: Some(label.into()),
        }
    }

    /// Set the spacing around the separator (top/bottom for horizontal, left/right for vertical).
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }
}

impl Widget for Separator {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = theme::current(ui.ctx());
        let p = &theme.palette;
        let t = &theme.typography;

        match self.direction {
            SeparatorDirection::Horizontal => {
                ui.add_space(self.spacing / 2.0);
                let width = ui.available_width().max(0.0);

                if let Some(label) = self.label {
                    let font_id = FontId::new(t.xs, egui::FontFamily::Name(crate::fonts::MEDIUM.into()));
                    let galley = ui
                        .scope(|ui| {
                            ui.style_mut().visuals.override_text_color = Some(p.text_secondary);
                            label.into_galley(ui, Some(egui::TextWrapMode::Extend), f32::INFINITY, font_id)
                        })
                        .inner;

                    let height = galley.size().y.max(16.0);
                    let (rect, response) = ui.allocate_exact_size(vec2(width, height), Sense::hover());
                    let mid_y = rect.center().y;

                    let label_w = galley.size().x;
                    let label_pad = 10.0;
                    let total_label_space = label_w + label_pad * 2.0;

                    let left_line_end = (rect.center().x - total_label_space / 2.0).max(rect.left());
                    let right_line_start = (rect.center().x + total_label_space / 2.0).min(rect.right());

                    // Left line
                    if left_line_end > rect.left() {
                        ui.painter().hline(rect.left()..=left_line_end, mid_y, Stroke::new(1.0, p.border));
                    }
                    // Right line
                    if right_line_start < rect.right() {
                        ui.painter().hline(right_line_start..=rect.right(), mid_y, Stroke::new(1.0, p.border));
                    }

                    // Text label centered
                    let text_pos = pos2(rect.center().x - label_w / 2.0, rect.center().y - galley.size().y / 2.0);
                    ui.painter().galley(text_pos, galley, p.text_secondary);

                    ui.add_space(self.spacing / 2.0);
                    response
                } else {
                    let (rect, response) = ui.allocate_exact_size(vec2(width, 1.0), Sense::hover());
                    ui.painter().hline(rect.x_range(), rect.center().y, Stroke::new(1.0, p.border));
                    ui.add_space(self.spacing / 2.0);
                    response
                }
            }
            SeparatorDirection::Vertical => {
                ui.add_space(self.spacing / 2.0);
                let height = ui.available_height().max(16.0);
                let (rect, response) = ui.allocate_exact_size(vec2(1.0, height), Sense::hover());
                ui.painter().vline(rect.center().x, rect.y_range(), Stroke::new(1.0, p.border));
                ui.add_space(self.spacing / 2.0);
                response
            }
        }
    }
}
