//! Themed horizontal slider with label and value readout.

use std::ops::RangeInclusive;

use egui::{pos2, vec2, Color32, Rect, Response, Sense, Stroke, Ui, Widget, WidgetText};

use crate::theme;
use crate::util::{mix, radius};
use super::Size;

/// A themed horizontal slider.
///
/// ```
/// # use oxida::prelude::*;
/// # fn demo(ui: &mut egui::Ui, mut volume: f32) {
/// ui.oxida()
///     .slider(&mut volume, 0.0..=100.0)
///     .label("Volume")
///     .suffix("%")
///     .ui(ui);
/// # }
/// ```
pub struct Slider<'v> {
    value: &'v mut f32,
    range: RangeInclusive<f32>,
    label: Option<WidgetText>,
    suffix: String,
    show_value: bool,
    step: Option<f32>,
}

impl<'v> Slider<'v> {
    /// Create a slider bound to `value` within `range`.
    pub fn new(value: &'v mut f32, range: RangeInclusive<f32>) -> Self {
        Self {
            value,
            range,
            label: None,
            suffix: String::new(),
            show_value: true,
            step: None,
        }
    }

    /// Label shown to the left of the track.
    pub fn label(mut self, label: impl Into<WidgetText>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Suffix appended to the value readout (e.g. `"%"`, `" px"`).
    pub fn suffix(mut self, suffix: impl Into<String>) -> Self {
        self.suffix = suffix.into();
        self
    }

    /// Show/hide the value readout. Defaults to `true`.
    pub fn show_value(mut self, show_value: bool) -> Self {
        self.show_value = show_value;
        self
    }

    /// Snap the value to multiples of `step`.
    pub fn step(mut self, step: f32) -> Self {
        self.step = Some(step);
        self
    }
}

impl Widget for Slider<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = theme::current(ui.ctx());
        let p = &theme.palette;
        let m = &theme.metrics;

        let height = m.control_height_md;
        let track_h = 4.0;
        let knob_r = 7.0;
        let gap = m.spacing_unit * 2.0;
        let value_font = theme.typography.font_id(Size::Sm);

        // Move the pieces out so we can measure before painting.
        let Slider {
            value,
            range,
            label,
            suffix,
            show_value,
            step,
        } = self;

        let (start, end) = (*range.start(), *range.end());
        let span = end - start;

        // Sanitize: the value must stay inside the range even if mutated
        // externally (or by a previous theme/range change).
        let clamped = value.clamp(start.min(end), start.max(end));
        if (clamped - *value).abs() > f32::EPSILON {
            *value = clamped;
        }

        let format_value = |v: f32| {
            let decimals = if span < 1.0 {
                2
            } else if span <= 10.0 {
                1
            } else {
                0
            };
            format!("{v:.decimals$}{suffix}")
        };

        let label_galley = label.map(|l| {
            l.into_galley(
                ui,
                Some(egui::TextWrapMode::Extend),
                f32::INFINITY,
                value_font.clone(),
            )
        });
        let value_galley = if show_value {
            Some(
                WidgetText::from(format_value(*value)).into_galley(
                    ui,
                    Some(egui::TextWrapMode::Extend),
                    f32::INFINITY,
                    value_font.clone(),
                ),
            )
        } else {
            None
        };

        let label_w = label_galley
            .as_ref()
            .map(|g| g.size().x + gap)
            .unwrap_or(0.0);
        let value_w = value_galley
            .as_ref()
            .map(|g| g.size().x + gap)
            .unwrap_or(0.0);

        let (rect, mut response) =
            ui.allocate_exact_size(vec2(ui.available_width(), height), Sense::click_and_drag());

        // Sub-rects: [label] [track .......] [value]
        let track_w = (rect.width() - label_w - value_w).max(knob_r * 2.0);
        let track = Rect::from_center_size(
            pos2(rect.left() + label_w + track_w / 2.0, rect.center().y),
            vec2(track_w, track_h),
        );

        // Interaction: the value follows the pointer while pressed or dragging.
        let enabled = ui.is_enabled();
        if enabled && (response.is_pointer_button_down_on() || response.dragged()) {
            if let Some(pos) = response.interact_pointer_pos() {
                let mut t = if track_w > 0.0 {
                    (pos.x - track.left()) / track_w
                } else {
                    0.0
                };
                t = t.clamp(0.0, 1.0);
                let mut v = start + t * span;
                if let Some(step) = step.filter(|s| *s > 0.0) {
                    v = ((v - start) / step).round() * step + start;
                }
                v = v.clamp(start.min(end), start.max(end));
                if (v - *value).abs() > f32::EPSILON {
                    *value = v;
                    response.mark_changed();
                }
            }
        }

        // Paint.
        let painter = ui.painter_at(rect);
        let hover_t = ui
            .ctx()
            .animate_bool_with_time(response.id, response.hovered() && enabled, 0.1);
        let pressed = enabled && response.is_pointer_button_down_on();

        // Track + filled portion.
        let track_round = radius(track_h / 2.0);
        let track_color = if enabled {
            mix(p.subtle, p.border, 0.6)
        } else {
            crate::util::dim(p.subtle, p.window_bg, 0.5)
        };
        painter.rect_filled(track, track_round, track_color);

        let t = if span > 0.0 {
            (*value - start) / span
        } else {
            0.0
        };
        let t = t.clamp(0.0, 1.0);
        let knob_x = track.left() + t * track_w;
        let filled = Rect::from_min_max(track.min, pos2(knob_x, track.max.y));
        let fill_color = if enabled {
            p.accent
        } else {
            crate::util::dim(p.accent, p.window_bg, 0.6)
        };
        painter.rect_filled(filled, track_round, fill_color);

        // Knob.
        let knob_r = knob_r + hover_t * 1.5 + if pressed { 1.0 } else { 0.0 };
        let knob_center = pos2(knob_x, rect.center().y);
        if pressed {
            // Soft shadow while dragging.
            painter.circle_filled(
                pos2(knob_center.x, knob_center.y + 1.5),
                knob_r,
                Color32::from_black_alpha(40),
            );
        }
        let knob_fill = if enabled {
            p.surface
        } else {
            crate::util::dim(p.surface, p.window_bg, 0.5)
        };
        let knob_stroke = if enabled {
            Stroke::new(m.border_width, p.border_strong)
        } else {
            Stroke::new(m.border_width, p.border)
        };
        painter.circle(knob_center, knob_r, knob_fill, knob_stroke);

        // Label + value.
        if let Some(g) = label_galley {
            let pos = pos2(rect.left(), rect.center().y - g.size().y / 2.0);
            painter.galley(pos, g, p.text_secondary);
        }
        if let Some(g) = value_galley {
            let pos = pos2(
                rect.right() - g.size().x,
                rect.center().y - g.size().y / 2.0,
            );
            painter.galley(pos, g, p.text_primary);
        }

        response
    }
}
