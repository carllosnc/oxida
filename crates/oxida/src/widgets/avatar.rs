//! Themed Avatar widget.

use egui::{pos2, vec2, FontFamily, FontId, Response, Sense, Stroke, Ui, Widget, WidgetText};

use super::Size;
use crate::theme;

/// Online/presence status for an [`Avatar`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AvatarStatus {
    /// Online / active (green dot).
    Online,
    /// Offline / inactive (muted dot).
    Offline,
    /// Busy / do not disturb (red dot).
    Busy,
    /// Away / idle (amber/yellow dot).
    Away,
}

/// A circular user avatar with fallback initials and optional presence dot.
///
/// ```
/// # use oxida::prelude::*;
/// # fn demo(ui: &mut egui::Ui) {
/// Avatar::initials("Alex Martins")
///     .status(AvatarStatus::Online)
///     .ui(ui);
/// # }
/// ```
#[derive(Clone, Debug)]
pub struct Avatar {
    initials: String,
    size: Size,
    status: Option<AvatarStatus>,
}

impl Avatar {
    /// Create an avatar with explicit text/initials.
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            initials: text.into(),
            size: Size::Md,
            status: None,
        }
    }

    /// Automatically extract up to two uppercase initials from a name (e.g. "Alex Martins" -> "AM").
    pub fn initials(name: &str) -> Self {
        let parts: Vec<&str> = name.split_whitespace().collect();
        let initials = match parts.as_slice() {
            [] => String::new(),
            [first] => first.chars().next().map(|c| c.to_uppercase().to_string()).unwrap_or_default(),
            [first, .., last] => {
                let f = first.chars().next().map(|c| c.to_uppercase().to_string()).unwrap_or_default();
                let l = last.chars().next().map(|c| c.to_uppercase().to_string()).unwrap_or_default();
                format!("{}{}", f, l)
            }
        };

        Self::new(initials)
    }

    /// Set the size of the avatar. Defaults to [`Size::Md`].
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    /// Add a presence status indicator dot.
    pub fn status(mut self, status: AvatarStatus) -> Self {
        self.status = Some(status);
        self
    }
}

impl Widget for Avatar {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = theme::current(ui.ctx());
        let p = &theme.palette;
        let m = &theme.metrics;

        let (dim, font_size, dot_radius) = match self.size {
            Size::Sm => (26.0, 11.0, 3.5),
            Size::Md => (36.0, 13.5, 4.5),
            Size::Lg => (48.0, 18.0, 6.0),
        };

        let (rect, response) = ui.allocate_exact_size(vec2(dim, dim), Sense::hover());
        let center = rect.center();
        let radius = dim / 2.0;

        // Avatar background & subtle border
        ui.painter().circle_filled(center, radius, p.subtle);
        ui.painter().circle_stroke(
            center,
            radius,
            Stroke::new(m.border_width, p.border),
        );

        // Initials text
        let font_id = FontId::new(font_size, FontFamily::Name(crate::fonts::SEMI_BOLD.into()));
        let galley = ui
            .scope(|ui| {
                ui.style_mut().visuals.override_text_color = Some(p.text_primary);
                WidgetText::from(self.initials).into_galley(ui, Some(egui::TextWrapMode::Extend), f32::INFINITY, font_id)
            })
            .inner;

        let text_pos = pos2(center.x - galley.size().x / 2.0, center.y - galley.size().y / 2.0);
        ui.painter().galley(text_pos, galley, p.text_primary);

        // Presence indicator dot (bottom-right)
        if let Some(status) = self.status {
            let dot_color = match status {
                AvatarStatus::Online => p.success,
                AvatarStatus::Offline => p.text_disabled,
                AvatarStatus::Busy => p.danger,
                AvatarStatus::Away => p.warning,
            };

            let offset = radius * 0.70;
            let dot_center = pos2(center.x + offset, center.y + offset);

            // Ring cutout behind dot matching surface color
            ui.painter().circle_filled(dot_center, dot_radius + 1.5, p.window_bg);
            // Dot
            ui.painter().circle_filled(dot_center, dot_radius, dot_color);
        }

        response
    }
}
