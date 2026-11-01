//! Oxida's design-token system.
//!
//! A [`Theme`] is a bundle of semantic tokens ([`Palette`], [`Typography`],
//! [`Metrics`]) describing *how things should look*, never *what to draw*.
//! Widgets consume the active theme at paint time, so switching themes is a
//! one-liner and works live.

mod metrics;
mod palette;
mod typography;

pub use metrics::Metrics;
pub use palette::Palette;
pub use typography::Typography;

use std::sync::Arc;

/// A complete visual theme.
///
/// Built-in themes: [`Theme::light`] and [`Theme::dark`].
/// Create your own by starting from one of them and overriding tokens:
///
/// ```
/// use oxida::Theme;
///
/// let mut theme = Theme::light();
/// theme.palette.accent = egui::Color32::from_rgb(79, 70, 229); // indigo
/// theme.metrics.radius_md = 8.0;
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct Theme {
    /// Human-readable name (e.g. `"Light"`).
    pub name: &'static str,
    /// Whether this theme targets dark mode (affects a few egui internals).
    pub dark_mode: bool,
    /// Semantic colors.
    pub palette: Palette,
    /// Font sizes.
    pub typography: Typography,
    /// Spacing, sizing and rounding.
    pub metrics: Metrics,
}

impl Default for Theme {
    fn default() -> Self {
        Self::light()
    }
}

impl Theme {
    /// A clean, neutral light theme.
    pub fn light() -> Self {
        Self {
            name: "Light",
            dark_mode: false,
            palette: Palette::light(),
            typography: Typography::default(),
            metrics: Metrics::default(),
        }
    }

    /// A clean, neutral dark theme.
    pub fn dark() -> Self {
        Self {
            name: "Dark",
            dark_mode: true,
            palette: Palette::dark(),
            ..Self::light()
        }
    }

    /// Apply this theme to `ctx`.
    ///
    /// * Restyles built-in egui widgets (panels, menus, `TextEdit`, ...) to match.
    /// * Stores the theme so every Oxida widget picks it up this frame.
    ///
    /// Cheap enough to call every frame, but typically you call it once at
    /// startup and again whenever the user switches theme.
    pub fn apply(&self, ctx: &egui::Context) {
        let egui_theme = if self.dark_mode {
            egui::Theme::Dark
        } else {
            egui::Theme::Light
        };
        ctx.set_style_of(egui_theme, crate::style::to_egui_style(self));
        ctx.set_theme(egui_theme);
        ctx.data_mut(|data| data.insert_temp(theme_id(), Arc::new(self.clone())));
    }

    /// A ready-to-use egui [`Frame`](egui::Frame) styled as a card/surface
    /// of this theme — handy for grouping content in galleries and apps.
    pub fn card_frame(&self) -> egui::Frame {
        egui::Frame {
            fill: self.palette.surface,
            stroke: egui::Stroke::new(self.metrics.border_width, self.palette.border),
            corner_radius: egui::CornerRadius::same(self.metrics.radius_lg as u8),
            inner_margin: egui::Margin::same((self.metrics.spacing_unit * 4.0) as i8),
            ..Default::default()
        }
    }
}

fn theme_id() -> egui::Id {
    egui::Id::unique("oxida-theme")
}

/// The active theme, or the default light theme when none was applied.
pub(crate) fn current(ctx: &egui::Context) -> Arc<Theme> {
    ctx.data(|data| data.get_temp(theme_id()))
        .unwrap_or_else(|| Arc::new(Theme::light()))
}
