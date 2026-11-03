//! Themed Segmented Control / Tabs widget.

use egui::{pos2, vec2, FontFamily, FontId, Response, Sense, Ui, Widget, WidgetText};

use super::Size;
use crate::theme;
use crate::util::{mix, radius};

/// A segmented control / tab bar styled after shadcn/ui and modern desktop OSes.
///
/// ```
/// # use oxida::prelude::*;
/// # fn demo(ui: &mut egui::Ui) {
/// let mut current = 0;
/// Tabs::new(&mut current, ["Visão Geral", "Métricas", "Configurações"]).ui(ui);
/// # }
/// ```
pub struct Tabs<'a, T: PartialEq + Clone> {
    current_value: &'a mut T,
    items: Vec<(T, WidgetText)>,
    size: Size,
}

impl<'a, T: PartialEq + Clone> Tabs<'a, T> {
    /// Create tabs from an array/slice of options.
    pub fn new<I, S>(current_value: &'a mut T, items: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<WidgetText>,
        T: From<usize>,
    {
        let items: Vec<(T, WidgetText)> = items
            .into_iter()
            .enumerate()
            .map(|(i, text)| (T::from(i), text.into()))
            .collect();

        Self {
            current_value,
            items,
            size: Size::Md,
        }
    }

    /// Create tabs with explicit values and labels.
    pub fn with_values(
        current_value: &'a mut T,
        items: impl IntoIterator<Item = (T, impl Into<WidgetText>)>,
    ) -> Self {
        let items = items
            .into_iter()
            .map(|(val, text)| (val, text.into()))
            .collect();

        Self {
            current_value,
            items,
            size: Size::Md,
        }
    }

    /// Set the size of the tabs. Defaults to [`Size::Md`].
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }
}

impl<'a, T: PartialEq + Clone + 'static> Widget for Tabs<'a, T> {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = theme::current(ui.ctx());
        let p = &theme.palette;
        let m = &theme.metrics;
        let t = &theme.typography;

        let height = match self.size {
            Size::Sm => 28.0,
            Size::Md => 36.0,
            Size::Lg => 42.0,
        };

        let font_size = match self.size {
            Size::Sm => t.xs,
            Size::Md => t.md - 0.5,
            Size::Lg => t.md + 1.0,
        };

        let track_padding = 3.0;
        let item_height = height - track_padding * 2.0;
        let tab_rounding = radius(m.radius_md - 1.0);
        let track_rounding = radius(m.radius_md);

        let font_id = FontId::new(font_size, FontFamily::Name(crate::fonts::MEDIUM.into()));

        // Pre-calculate galleys and widths for all tabs
        let mut tab_widths = Vec::with_capacity(self.items.len());
        let mut tab_galleys = Vec::with_capacity(self.items.len());

        let min_tab_w = match self.size {
            Size::Sm => 48.0,
            Size::Md => 64.0,
            Size::Lg => 80.0,
        };
        let pad_x = match self.size {
            Size::Sm => 10.0,
            Size::Md => 14.0,
            Size::Lg => 18.0,
        };

        for (val, label) in &self.items {
            let is_selected = *val == *self.current_value;
            let text_color = if is_selected { p.text_primary } else { p.text_secondary };

            let galley = ui
                .scope(|ui| {
                    ui.style_mut().visuals.override_text_color = Some(text_color);
                    label.clone().into_galley(ui, Some(egui::TextWrapMode::Extend), f32::INFINITY, font_id.clone())
                })
                .inner;

            let w = (galley.size().x + pad_x * 2.0).max(min_tab_w);
            tab_widths.push(w);
            tab_galleys.push(galley);
        }

        let total_tabs_w: f32 = tab_widths.iter().sum();
        let total_w = total_tabs_w + track_padding * 2.0;

        let desired = vec2(total_w, height);
        let (rect, mut overall_response) = ui.allocate_exact_size(desired, Sense::hover());

        // Draw track
        ui.painter().rect_filled(rect, track_rounding, p.subtle);

        // Draw each tab
        let mut curr_x = rect.left() + track_padding;
        let tab_top = rect.top() + track_padding;

        for (idx, (val, _)) in self.items.into_iter().enumerate() {
            let tab_w = tab_widths[idx];
            let tab_rect = egui::Rect::from_min_size(pos2(curr_x, tab_top), vec2(tab_w, item_height));
            let tab_id = ui.make_persistent_id(idx).with("tab_item");
            let tab_resp = ui.interact(tab_rect, tab_id, Sense::click());

            let is_selected = val == *self.current_value;

            if tab_resp.clicked() && !is_selected {
                *self.current_value = val.clone();
                overall_response.mark_changed();
            }

            // Background of active tab vs hover (clean borderless pill)
            if is_selected {
                ui.painter().rect_filled(tab_rect, tab_rounding, p.surface);
            } else if tab_resp.hovered() {
                let hover_bg = mix(p.subtle, p.surface, 0.45);
                ui.painter().rect_filled(tab_rect, tab_rounding, hover_bg);
            }

            // Draw text
            let galley = &tab_galleys[idx];
            let text_color = if is_selected || tab_resp.hovered() {
                p.text_primary
            } else {
                p.text_secondary
            };

            let text_pos = pos2(
                tab_rect.center().x - galley.size().x / 2.0,
                tab_rect.center().y - galley.size().y / 2.0,
            );
            ui.painter().galley(text_pos, galley.clone(), text_color);

            curr_x += tab_w;
        }

        overall_response
    }
}
