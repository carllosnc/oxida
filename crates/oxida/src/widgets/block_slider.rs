//! Themed block slider / filled scrubber pill widget with embedded value readout.

use std::ops::RangeInclusive;

use egui::{
    pos2, vec2, Color32, CornerRadius, CursorIcon, FontFamily, FontId, Key, Rect, Response, Sense,
    Stroke, StrokeKind, Ui, Widget, WidgetText,
};

use super::Size;
use crate::theme;
use crate::util::{dim, mix, radius};

/// A modern block slider / filled scrubber pill with an embedded value readout.
///
/// Styled after modern desktop control center sliders (macOS, iOS, Linear).
/// The entire pill is interactive — clicking or dragging anywhere smoothly scrubs the value.
///
/// ```
/// # use oxida::prelude::*;
/// # fn demo(ui: &mut egui::Ui, mut val: f32) {
/// BlockSlider::percentage(&mut val).ui(ui);
/// # }
/// ```
pub struct BlockSlider<'v> {
    value: &'v mut f32,
    range: RangeInclusive<f32>,
    step: Option<f32>,
    label: Option<WidgetText>,
    prefix: String,
    suffix: String,
    show_value: bool,
    height: Option<f32>,
    width: Option<f32>,
    size: Size,
    icon: Option<&'static str>,
    fill_color: Option<Color32>,
    use_accent: bool,
    show_needle: bool,
}

impl<'v> BlockSlider<'v> {
    /// Create a block slider bound to `value` within `range`.
    pub fn new(value: &'v mut f32, range: RangeInclusive<f32>) -> Self {
        Self {
            value,
            range,
            step: None,
            label: None,
            prefix: String::new(),
            suffix: String::new(),
            show_value: true,
            height: None,
            width: None,
            size: Size::Md,
            icon: None,
            fill_color: None,
            use_accent: false,
            show_needle: false,
        }
    }

    /// Convenience constructor for a 0% to 100% percentage slider.
    pub fn percentage(value: &'v mut f32) -> Self {
        Self::new(value, 0.0..=100.0).suffix("%")
    }

    /// Label shown inside the pill before the value.
    pub fn label(mut self, label: impl Into<WidgetText>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Prefix prepended to the numeric value readout.
    pub fn prefix(mut self, prefix: impl Into<String>) -> Self {
        self.prefix = prefix.into();
        self
    }

    /// Suffix appended to the value readout (e.g. `"%"`, `" dB"`, `" px"`).
    pub fn suffix(mut self, suffix: impl Into<String>) -> Self {
        self.suffix = suffix.into();
        self
    }

    /// Snap the value to multiples of `step`.
    pub fn step(mut self, step: f32) -> Self {
        self.step = Some(step);
        self
    }

    /// Show/hide the numeric value readout. Defaults to `true`.
    pub fn show_value(mut self, show_value: bool) -> Self {
        self.show_value = show_value;
        self
    }

    /// Set control size. Defaults to [`Size::Md`].
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    /// Custom height in pixels.
    pub fn height(mut self, height: f32) -> Self {
        self.height = Some(height);
        self
    }

    /// Custom explicit width in pixels.
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// Leading icon glyph placed before label/value (e.g. from [`crate::icons`]).
    pub fn icon(mut self, icon: &'static str) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Custom fill color for the active progress bar.
    pub fn fill(mut self, color: Color32) -> Self {
        self.fill_color = Some(color);
        self
    }

    /// Use the theme accent color for the filled portion.
    pub fn accent(mut self) -> Self {
        self.use_accent = true;
        self
    }

    /// Show or hide the vertical needle indicator. Defaults to `false`.
    pub fn needle(mut self, show: bool) -> Self {
        self.show_needle = show;
        self
    }
}

impl Widget for BlockSlider<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = theme::current(ui.ctx());
        let p = &theme.palette;
        let m = &theme.metrics;
        let t = &theme.typography;

        let (default_h, default_r, default_font, default_pad) = match self.size {
            Size::Sm => (16.0, (m.radius_sm - 1.0).max(2.0), (t.xs - 2.0).max(9.0), 6.0),
            Size::Md => (20.0, m.radius_sm, (t.xs - 0.5).max(10.0), 8.0),
            Size::Lg => (26.0, m.radius_md, t.sm, 10.0),
        };

        let height = self.height.unwrap_or(default_h);
        let corner_r = if self.height.is_some() {
            (height * 0.25).min(m.radius_md)
        } else {
            default_r
        };
        let track_rounding = radius(corner_r);

        let font_size = if let Some(h) = self.height {
            (h * 0.55).min(default_font)
        } else {
            default_font
        };

        let left_pad = if let Some(h) = self.height {
            (h * 0.4).max(6.0)
        } else {
            default_pad
        };

        let (start, end) = (*self.range.start(), *self.range.end());
        let span = end - start;

        // Sanitize value within range
        let clamped = self.value.clamp(start.min(end), start.max(end));
        if (clamped - *self.value).abs() > f32::EPSILON {
            *self.value = clamped;
        }

        let width = self.width.unwrap_or_else(|| ui.available_width().max(80.0));
        let (rect, mut response) = ui.allocate_exact_size(vec2(width, height), Sense::click_and_drag());

        let enabled = ui.is_enabled();

        // Keyboard navigation when focused (WCAG accessibility)
        if enabled && response.has_focus() {
            let step_size = self
                .step
                .unwrap_or(if span.abs() > 10.0 { 1.0 } else { span.abs() / 100.0 });
            let mut delta = 0.0;
            if ui.input(|i| i.key_pressed(Key::ArrowLeft) || i.key_pressed(Key::ArrowDown)) {
                delta -= step_size;
            }
            if ui.input(|i| i.key_pressed(Key::ArrowRight) || i.key_pressed(Key::ArrowUp)) {
                delta += step_size;
            }
            if ui.input(|i| i.key_pressed(Key::Home)) {
                *self.value = start;
                response.mark_changed();
            }
            if ui.input(|i| i.key_pressed(Key::End)) {
                *self.value = end;
                response.mark_changed();
            }
            if delta != 0.0 {
                *self.value = (*self.value + delta).clamp(start.min(end), start.max(end));
                response.mark_changed();
            }
        }

        // Pointer click & drag interaction
        if enabled && (response.is_pointer_button_down_on() || response.dragged()) {
            if let Some(pos) = response.interact_pointer_pos() {
                let mut frac = if rect.width() > 0.0 {
                    (pos.x - rect.left()) / rect.width()
                } else {
                    0.0
                };
                frac = frac.clamp(0.0, 1.0);
                let mut v = start + frac * span;
                if let Some(step) = self.step.filter(|s| *s > 0.0) {
                    v = ((v - start) / step).round() * step + start;
                }
                v = v.clamp(start.min(end), start.max(end));
                if (v - *self.value).abs() > f32::EPSILON {
                    *self.value = v;
                    response.mark_changed();
                }
            }
        }

        if response.hovered() && enabled {
            ui.output_mut(|o| o.cursor_icon = CursorIcon::ResizeHorizontal);
        }

        // Progress fraction in 0.0..=1.0
        let frac = if span.abs() > 0.0 {
            ((*self.value - start) / span).clamp(0.0, 1.0)
        } else {
            0.0
        };

        // Painting
        let painter = ui.painter_at(rect);

        // Track background
        let track_bg = if enabled {
            if response.hovered() {
                mix(p.subtle, p.surface, 0.25)
            } else {
                p.subtle
            }
        } else {
            dim(p.subtle, p.window_bg, 0.5)
        };
        painter.rect_filled(rect, track_rounding, track_bg);

        // Fill color
        let base_fill = if self.use_accent {
            p.accent
        } else if let Some(custom) = self.fill_color {
            custom
        } else {
            // Refined neutral gray matching modern control center styles
            mix(p.subtle, p.text_secondary, 0.42)
        };

        let fill_color = if enabled {
            if response.is_pointer_button_down_on() || response.dragged() {
                mix(base_fill, p.text_primary, 0.08)
            } else if response.hovered() {
                mix(base_fill, p.text_primary, 0.04)
            } else {
                base_fill
            }
        } else {
            dim(base_fill, p.window_bg, 0.5)
        };

        let fill_w = (rect.width() * frac).max(0.0);
        let fill_end_x = rect.left() + fill_w;

        // Draw filled region with rounded corners
        if frac > 0.002 {
            let filled_rect = Rect::from_min_max(rect.min, pos2(fill_end_x, rect.max.y));
            let r_u8 = corner_r.round() as u8;
            let fill_rounding = if frac >= 0.98 {
                CornerRadius::same(r_u8)
            } else {
                CornerRadius {
                    nw: r_u8,
                    sw: r_u8,
                    ne: 0,
                    se: 0,
                }
            };
            painter.rect_filled(filled_rect, fill_rounding, fill_color);
        }

        // Draw optional needle indicator (the vertical pip marking position)
        if self.show_needle {
            let needle_w = match self.size {
                Size::Sm => 2.0,
                Size::Md => 2.5,
                Size::Lg => 3.0,
            };
            let needle_h = (height * 0.6).round();
            let min_needle_x = rect.left() + (needle_w / 2.0) + 3.0;
            let max_needle_x = rect.right() - (needle_w / 2.0) - 3.0;
            let needle_x = fill_end_x.clamp(min_needle_x, max_needle_x);

            let needle_rect =
                Rect::from_center_size(pos2(needle_x, rect.center().y), vec2(needle_w, needle_h));
            let needle_color = if enabled {
                mix(p.text_primary, p.surface, 0.06)
            } else {
                p.text_disabled
            };
            painter.rect_filled(needle_rect, radius(needle_w / 2.0), needle_color);
        }

        // Format embedded label and value readout
        let decimals = if span.abs() < 1.0 {
            2
        } else if span.abs() <= 10.0 {
            1
        } else {
            0
        };

        let value_str = format!("{}{:.decimals$}{}", self.prefix, *self.value, self.suffix);
        let display_text = match (&self.label, self.show_value) {
            (Some(lbl), true) => format!("{} {}", lbl.text(), value_str),
            (Some(lbl), false) => lbl.text().to_string(),
            (None, true) => value_str,
            (None, false) => String::new(),
        };

        let text_color = if enabled {
            p.text_primary
        } else {
            p.text_disabled
        };

        let mut cur_x = rect.left() + left_pad;

        // Draw icon if configured
        if let Some(icon_glyph) = self.icon {
            let icon_font = FontId::new(
                font_size + 1.0,
                FontFamily::Name(crate::fonts::REGULAR.into()),
            );
            let icon_galley = ui
                .scope(|ui| {
                    ui.style_mut().visuals.override_text_color = Some(text_color);
                    WidgetText::from(icon_glyph).into_galley(
                        ui,
                        Some(egui::TextWrapMode::Extend),
                        f32::INFINITY,
                        icon_font,
                    )
                })
                .inner;
            let icon_w = icon_galley.size().x;
            let icon_pos = pos2(cur_x, rect.center().y - icon_galley.size().y / 2.0);
            painter.galley(icon_pos, icon_galley, text_color);
            cur_x += icon_w + 4.0;
        }

        // Draw embedded text inside pill
        if !display_text.is_empty() {
            let font_id = FontId::new(
                font_size,
                FontFamily::Name(crate::fonts::SEMI_BOLD.into()),
            );
            let text_galley = ui
                .scope(|ui| {
                    ui.style_mut().visuals.override_text_color = Some(text_color);
                    WidgetText::from(display_text).into_galley(
                        ui,
                        Some(egui::TextWrapMode::Truncate),
                        (rect.right() - cur_x - left_pad).max(0.0),
                        font_id,
                    )
                })
                .inner;
            let text_pos = pos2(cur_x, rect.center().y - text_galley.size().y / 2.0);
            painter.galley(text_pos, text_galley, text_color);
        }

        // Focus ring (WCAG 2.4.7 / 2.4.11)
        if response.has_focus() {
            painter.rect_stroke(
                rect.expand(m.focus_ring_offset),
                radius(corner_r + m.focus_ring_offset),
                Stroke::new(m.focus_ring_width, p.accent),
                StrokeKind::Outside,
            );
        }

        response
    }
}
