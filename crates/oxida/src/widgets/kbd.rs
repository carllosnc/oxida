//! Keyboard shortcut keycap widget (`Kbd`).
//!
//! Visual representation of keyboard shortcuts for menus, buttons, search bars,
//! tooltips, and command palettes. Styled after modern desktop interfaces (shadcn/ui, Linear).

use egui::{
    pos2, vec2, FontFamily, FontId, Response, Sense, Stroke, Ui, Widget, WidgetText,
};

use super::Size;
use crate::theme;
use crate::util::{mix, radius};

/// A visual keyboard shortcut keycap (`<kbd>`).
///
/// Can display a single key (e.g. `Ctrl`, `⌘`, `K`, `Enter`) or a chord
/// of multiple keys (e.g. `Ctrl + Shift + P`) with tactile desktop styling.
///
/// ```
/// # use oxida::prelude::*;
/// # fn demo(ui: &mut egui::Ui) {
/// Kbd::new("Ctrl").ui(ui);
/// Kbd::shortcut("Ctrl", "K").ui(ui);
/// # }
/// ```
#[derive(Clone, Debug)]
pub struct Kbd {
    keys: Vec<String>,
    size: Size,
    separator: Option<String>,
}

impl Kbd {
    /// Create a keycap with a single key label.
    pub fn new(key: impl Into<String>) -> Self {
        Self {
            keys: vec![key.into()],
            size: Size::Sm,
            separator: Some("+".to_string()),
        }
    }

    /// Create a shortcut chord of two keys (e.g. `Ctrl` + `K`).
    pub fn shortcut(mod_key: impl Into<String>, key: impl Into<String>) -> Self {
        Self {
            keys: vec![mod_key.into(), key.into()],
            size: Size::Sm,
            separator: Some("+".to_string()),
        }
    }

    /// Create a shortcut keycap from an array/slice of keys.
    pub fn keys<I, S>(keys: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self {
            keys: keys.into_iter().map(Into::into).collect(),
            size: Size::Sm,
            separator: Some("+".to_string()),
        }
    }

    /// Set the size of the keycap. Defaults to [`Size::Sm`].
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    /// Set custom separator text between chord keys (default: `"+"`).
    /// Pass `None` to display keys adjacent with spacing only.
    pub fn separator(mut self, sep: Option<impl Into<String>>) -> Self {
        self.separator = sep.map(Into::into);
        self
    }
}

impl Widget for Kbd {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = theme::current(ui.ctx());
        let p = &theme.palette;
        let m = &theme.metrics;

        let (font_size, min_height, h_pad, min_w) = match self.size {
            Size::Sm => (theme.typography.xs, 18.0_f32, 5.0_f32, 18.0_f32),
            Size::Md => (theme.typography.sm, 22.0_f32, 7.0_f32, 22.0_f32),
            Size::Lg => (theme.typography.md, 26.0_f32, 9.0_f32, 26.0_f32),
        };

        let font_id = FontId::new(
            font_size,
            FontFamily::Name(crate::fonts::MEDIUM.into()),
        );

        let sep_font_id = FontId::new(
            font_size - 1.0,
            FontFamily::Name(crate::fonts::REGULAR.into()),
        );

        // Pre-compute galleys and widths inside ui.scope (AGENTS.md override_text_color rule).
        let key_color = p.text_secondary;
        let sep_color = mix(p.text_secondary, p.border, 0.5);

        let mut key_galleys = Vec::with_capacity(self.keys.len());
        for key in &self.keys {
            let galley = ui.scope(|ui| {
                ui.style_mut().visuals.override_text_color = Some(key_color);
                WidgetText::from(key.as_str()).into_galley(
                    ui,
                    Some(egui::TextWrapMode::Truncate),
                    f32::INFINITY,
                    font_id.clone(),
                )
            }).inner;
            key_galleys.push(galley);
        }

        let sep_galley = self.separator.as_ref().map(|sep| {
            ui.scope(|ui| {
                ui.style_mut().visuals.override_text_color = Some(sep_color);
                WidgetText::from(sep.as_str()).into_galley(
                    ui,
                    Some(egui::TextWrapMode::Truncate),
                    f32::INFINITY,
                    sep_font_id,
                )
            }).inner
        });

        let key_spacing = 4.0_f32;
        let sep_spacing = 3.0_f32;

        // Calculate total bounding size
        let mut total_width = 0.0_f32;
        let mut key_widths = Vec::with_capacity(key_galleys.len());

        for (i, galley) in key_galleys.iter().enumerate() {
            let kw = (galley.size().x + h_pad * 2.0).max(min_w);
            key_widths.push(kw);
            total_width += kw;

            if i + 1 < key_galleys.len() {
                if let Some(ref sep_g) = sep_galley {
                    total_width += sep_spacing * 2.0 + sep_g.size().x;
                } else {
                    total_width += key_spacing;
                }
            }
        }

        let total_size = vec2(total_width, min_height);
        let (rect, response) = ui.allocate_exact_size(total_size, Sense::hover());

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();
            let mut curr_x = rect.min.x;

            let bg_color = p.subtle;
            let border_stroke = Stroke::new(m.border_width, mix(p.border, p.subtle, 0.3));
            let corner_r = radius(m.radius_sm);

            for (i, galley) in key_galleys.into_iter().enumerate() {
                let kw = key_widths[i];
                let key_rect = egui::Rect::from_min_size(
                    pos2(curr_x, rect.min.y),
                    vec2(kw, min_height),
                );

                // Subtle key surface + delicate border
                painter.rect(
                    key_rect,
                    corner_r,
                    bg_color,
                    border_stroke,
                    egui::StrokeKind::Inside,
                );

                // Center text in keycap
                let text_pos = pos2(
                    key_rect.center().x - galley.size().x * 0.5,
                    key_rect.center().y - galley.size().y * 0.5,
                );
                painter.galley(text_pos, galley, key_color);

                curr_x += kw;

                // Separator between keys
                if i + 1 < self.keys.len() {
                    if let Some(ref sep_g) = sep_galley {
                        curr_x += sep_spacing;
                        let sep_pos = pos2(
                            curr_x,
                            rect.center().y - sep_g.size().y * 0.5,
                        );
                        painter.galley(sep_pos, sep_g.clone(), sep_color);
                        curr_x += sep_g.size().x + sep_spacing;
                    } else {
                        curr_x += key_spacing;
                    }
                }
            }
        }

        response
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kbd_construction() {
        let kbd = Kbd::new("Ctrl");
        assert_eq!(kbd.keys, vec!["Ctrl"]);

        let chord = Kbd::shortcut("Ctrl", "K");
        assert_eq!(chord.keys, vec!["Ctrl", "K"]);

        let multi = Kbd::keys(["Ctrl", "Shift", "P"]).separator(None::<String>);
        assert_eq!(multi.keys.len(), 3);
        assert!(multi.separator.is_none());
    }
}
