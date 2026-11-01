//! Typography tokens.

use egui::{FontFamily, FontId, TextStyle};

/// Font sizes used by Oxida widgets.
///
/// Sizes map onto egui's [`TextStyle`] roles when the theme is applied:
/// `Small` → [`Typography::xs`], `Body`/`Button` → [`Typography::md`],
/// `Heading` → [`Typography::lg`], `Monospace` → [`Typography::sm`].
#[derive(Clone, Debug, PartialEq)]
pub struct Typography {
    /// Extra small: captions, badges, table headers.
    pub xs: f32,
    /// Small: secondary text, value readouts.
    pub sm: f32,
    /// Medium: body text, buttons, inputs (the default).
    pub md: f32,
    /// Large: section titles.
    pub lg: f32,
    /// Title: page/window titles.
    pub title: f32,
}

impl Default for Typography {
    fn default() -> Self {
        Self {
            xs: 11.0,
            sm: 12.0,
            md: 13.0,
            lg: 16.0,
            title: 20.0,
        }
    }
}

impl Typography {
    /// The [`FontId`] for a given Oxida size role.
    pub fn font_id(&self, size: crate::widgets::Size) -> FontId {
        let px = match size {
            crate::widgets::Size::Sm => self.xs,
            crate::widgets::Size::Md => self.md,
            crate::widgets::Size::Lg => self.lg,
        };
        FontId::new(px, FontFamily::Proportional)
    }

    /// Map the sizes onto egui's built-in [`TextStyle`] roles.
    pub(crate) fn text_styles(&self) -> impl Iterator<Item = (TextStyle, FontId)> {
        [
            (TextStyle::Small, FontId::new(self.xs, FontFamily::Proportional)),
            (TextStyle::Body, FontId::new(self.md, FontFamily::Proportional)),
            (TextStyle::Button, FontId::new(self.md, FontFamily::Proportional)),
            (TextStyle::Heading, FontId::new(self.lg, FontFamily::Proportional)),
            (TextStyle::Monospace, FontId::new(self.sm, FontFamily::Monospace)),
        ]
        .into_iter()
    }
}
