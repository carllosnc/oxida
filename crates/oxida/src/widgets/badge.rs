//! Themed Badge / Tag widget.

use egui::{pos2, vec2, Color32, FontFamily, FontId, Response, Sense, Ui, Widget, WidgetText};

use super::Size;
use crate::theme;
use crate::util::{mix, radius};

/// Visual variant of a [`Badge`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BadgeVariant {
    /// Accent background — primary emphasis.
    #[default]
    Default,
    /// Subtle background with a subtle border.
    Secondary,
    /// Transparent background with a distinct border.
    Outline,
    /// Destructive/danger state.
    Destructive,
    /// Positive/success state.
    Success,
    /// Warning/caution state.
    Warning,
}

/// A compact status badge, tag or chip styled after shadcn/ui.
///
/// ```
/// # use oxida::prelude::*;
/// # fn demo(ui: &mut egui::Ui) {
/// Badge::new("Ativo").variant(BadgeVariant::Success).ui(ui);
/// # }
/// ```
#[derive(Clone)]
pub struct Badge {
    text: WidgetText,
    variant: BadgeVariant,
    size: Size,
    dot: Option<Color32>,
    icon: Option<WidgetText>,
}

impl Badge {
    /// Create a new badge with the given text.
    pub fn new(text: impl Into<WidgetText>) -> Self {
        Self {
            text: text.into(),
            variant: BadgeVariant::Default,
            size: Size::Sm,
            dot: None,
            icon: None,
        }
    }

    /// Default primary badge.
    pub fn default_badge(text: impl Into<WidgetText>) -> Self {
        Self::new(text).variant(BadgeVariant::Default)
    }

    /// Secondary badge (subtle background).
    pub fn secondary(text: impl Into<WidgetText>) -> Self {
        Self::new(text).variant(BadgeVariant::Secondary)
    }

    /// Outline badge (transparent background with border).
    pub fn outline(text: impl Into<WidgetText>) -> Self {
        Self::new(text).variant(BadgeVariant::Outline)
    }

    /// Destructive badge for errors or danger.
    pub fn destructive(text: impl Into<WidgetText>) -> Self {
        Self::new(text).variant(BadgeVariant::Destructive)
    }

    /// Success badge for positive/active states.
    pub fn success(text: impl Into<WidgetText>) -> Self {
        Self::new(text).variant(BadgeVariant::Success)
    }

    /// Warning badge for warnings or pending states.
    pub fn warning(text: impl Into<WidgetText>) -> Self {
        Self::new(text).variant(BadgeVariant::Warning)
    }

    /// Set the visual variant.
    pub fn variant(mut self, variant: BadgeVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set the badge size. Defaults to [`Size::Sm`].
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    /// Show a small status indicator dot before the text.
    pub fn dot(mut self, color: Color32) -> Self {
        self.dot = Some(color);
        self
    }

    /// Add a leading icon to the badge.
    pub fn icon(mut self, icon: impl Into<WidgetText>) -> Self {
        self.icon = Some(icon.into());
        self
    }
}

impl Widget for Badge {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = theme::current(ui.ctx());
        let p = &theme.palette;
        let t = &theme.typography;

        let (bg, fg) = match self.variant {
            BadgeVariant::Default => (p.accent, p.on_accent),
            BadgeVariant::Secondary | BadgeVariant::Outline => (p.subtle, p.text_primary),
            BadgeVariant::Destructive => (
                mix(p.danger, p.surface, 0.80),
                p.danger,
            ),
            BadgeVariant::Success => (
                mix(p.success, p.surface, 0.80),
                p.success,
            ),
            BadgeVariant::Warning => (
                mix(p.warning, p.surface, 0.80),
                p.warning,
            ),
        };

        let height = match self.size {
            Size::Sm => 20.0,
            Size::Md => 24.0,
            Size::Lg => 28.0,
        };
        let font_size = match self.size {
            Size::Sm => t.xs - 0.5,
            Size::Md => t.xs + 0.5,
            Size::Lg => t.sm,
        };
        let pad_x = match self.size {
            Size::Sm => 7.0,
            Size::Md => 9.0,
            Size::Lg => 11.0,
        };

        let font_id = FontId::new(
            font_size,
            FontFamily::Name(crate::fonts::MEDIUM.into()),
        );

        let icon_font_id = FontId::new(
            font_size + 1.0,
            FontFamily::Name(crate::fonts::PHOSPHOR.into()),
        );

        let icon_galley = self.icon.map(|ic| {
            ui.scope(|ui| {
                ui.style_mut().visuals.override_text_color = Some(fg);
                ic.into_galley(ui, Some(egui::TextWrapMode::Extend), f32::INFINITY, icon_font_id)
            })
            .inner
        });

        let text_galley = ui
            .scope(|ui| {
                ui.style_mut().visuals.override_text_color = Some(fg);
                self.text.into_galley(ui, Some(egui::TextWrapMode::Extend), f32::INFINITY, font_id)
            })
            .inner;

        let dot_radius = match self.size {
            Size::Sm => 2.5,
            Size::Md => 3.0,
            Size::Lg => 3.5,
        };
        let dot_width = if self.dot.is_some() { dot_radius * 2.0 + 4.0 } else { 0.0 };
        let icon_width = if let Some(ref ig) = icon_galley { ig.size().x + 4.0 } else { 0.0 };

        let content_width = dot_width + icon_width + text_galley.size().x;
        let desired = vec2(content_width + pad_x * 2.0, height);
        let (rect, response) = ui.allocate_exact_size(desired, Sense::hover());

        let rounding = radius(height / 2.0); // Clean borderless pill shape
        ui.painter().rect_filled(rect, rounding, bg);

        let mut curr_x = rect.left() + pad_x;

        if let Some(dot_c) = self.dot {
            let dot_center = pos2(curr_x + dot_radius, rect.center().y);
            ui.painter().circle_filled(dot_center, dot_radius, dot_c);
            curr_x += dot_width;
        }

        if let Some(ref ig) = icon_galley {
            let icon_pos = pos2(curr_x, rect.center().y - ig.size().y / 2.0);
            ui.painter().galley(icon_pos, ig.clone(), fg);
            curr_x += icon_width;
        }

        let text_pos = pos2(curr_x, rect.center().y - text_galley.size().y / 2.0);
        ui.painter().galley(text_pos, text_galley, fg);

        response
    }
}
