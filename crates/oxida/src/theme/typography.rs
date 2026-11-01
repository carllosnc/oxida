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
    /// Font family used for [`TextStyle::Heading`].
    ///
    /// Defaults to the bundled Inter SemiBold
    /// ([`crate::fonts::SEMI_BOLD`]).
    pub heading_family: egui::FontFamily,
    /// Font family used for [`TextStyle::Button`] and every button label.
    ///
    /// Defaults to the proportional font (Inter Regular).
    pub button_family: egui::FontFamily,
}

impl Default for Typography {
    fn default() -> Self {
        Self {
            xs: 12.0,
            sm: 13.0,
            md: 14.0,
            lg: 24.0,
            title: 28.0,
            heading_family: egui::FontFamily::Name(crate::fonts::SEMI_BOLD.into()),
            button_family: egui::FontFamily::Name(crate::fonts::MEDIUM.into()),
        }
    }
}

impl Typography {
    /// The [`FontId`] for a given Oxida size role.
    pub fn font_id(&self, size: crate::widgets::Size) -> FontId {
        let px = match size {
            crate::widgets::Size::Sm => self.xs,
            crate::widgets::Size::Md => self.md,
            crate::widgets::Size::Lg => (self.lg - 6.0).max(self.md),
        };
        FontId::new(px, FontFamily::Proportional)
    }

    /// Font for page and window titles.
    pub fn title_font(&self) -> FontId {
        FontId::new(self.title, egui::FontFamily::Name(crate::fonts::BOLD.into()))
    }

    /// Font for section titles.
    pub fn heading_font(&self) -> FontId {
        FontId::new(self.lg, self.heading_family.clone())
    }

    /// Map the sizes onto egui's built-in [`TextStyle`] roles.
    pub(crate) fn text_styles(&self) -> impl Iterator<Item = (TextStyle, FontId)> {
        [
            (TextStyle::Small, FontId::new(self.xs, FontFamily::Proportional)),
            (TextStyle::Body, FontId::new(self.md, FontFamily::Proportional)),
            (TextStyle::Button, FontId::new(self.md, self.button_family.clone())),
            (
                TextStyle::Heading,
                FontId::new(self.lg, self.heading_family.clone()),
            ),
            (TextStyle::Monospace, FontId::new(self.sm, FontFamily::Monospace)),
        ]
        .into_iter()
    }
}
