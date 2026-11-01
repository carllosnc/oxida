//! Themed Select component inspired by shadcn/ui.

use egui::{
    containers::{Popup, PopupCloseBehavior},
    pos2, vec2, Color32, Id, Margin, Response, ScrollArea, Sense, Shadow, Stroke, StrokeKind, Ui,
    Widget, WidgetText,
};

use crate::theme;
use crate::util::radius;

/// An item in a [`Select`] dropdown.
#[derive(Clone, Debug)]
pub struct SelectItem<T: Clone + PartialEq> {
    pub value: T,
    pub label: WidgetText,
}

impl<T: Clone + PartialEq> SelectItem<T> {
    /// Create a new select option.
    pub fn new(value: T, label: impl Into<WidgetText>) -> Self {
        Self {
            value,
            label: label.into(),
        }
    }
}

/// A clean single-select dropdown inspired by shadcn/ui.
///
/// ```
/// # use oxida::prelude::*;
/// # fn demo(ui: &mut egui::Ui, mut choice: usize) {
/// Select::new("framework_select", &mut choice)
///     .placeholder("Selecione uma opção...")
///     .item(0, "Next.js")
///     .item(1, "SvelteKit")
///     .item(2, "Astro")
///     .ui(ui);
/// # }
/// ```
pub struct Select<'a, T: Clone + PartialEq> {
    id_salt: Id,
    selected: &'a mut T,
    items: Vec<SelectItem<T>>,
    placeholder: Option<WidgetText>,
    width: Option<f32>,
    enabled: bool,
}

impl<'a, T: Clone + PartialEq> Select<'a, T> {
    /// Create a new Select bound to `selected`.
    pub fn new(id_salt: impl std::hash::Hash + std::fmt::Debug, selected: &'a mut T) -> Self {
        Self {
            id_salt: Id::unique(id_salt),
            selected,
            items: Vec::new(),
            placeholder: None,
            width: None,
            enabled: true,
        }
    }

    /// Placeholder text shown when no item matches the current selection.
    pub fn placeholder(mut self, placeholder: impl Into<WidgetText>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    /// Set an explicit width for the select trigger and popup.
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// Add a selectable item.
    pub fn item(mut self, value: T, label: impl Into<WidgetText>) -> Self {
        self.items.push(SelectItem::new(value, label));
        self
    }

    /// Add multiple selectable items from an iterator of `(value, label)`.
    pub fn items(mut self, items: impl IntoIterator<Item = (T, impl Into<WidgetText>)>) -> Self {
        for (val, label) in items {
            self.items.push(SelectItem::new(val, label));
        }
        self
    }

    /// Enable or disable the select input.
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

impl<T: Clone + PartialEq + 'static> Widget for Select<'_, T> {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = theme::current(ui.ctx());
        let p = &theme.palette;
        let m = &theme.metrics;

        let width = self.width.unwrap_or(240.0).max(120.0);
        let height = m.control_height_md;
        let desired = vec2(width, height);

        let popup_id = self.id_salt.with("popup");
        let is_open = Popup::is_id_open(ui.ctx(), popup_id);

        let sense = if self.enabled {
            Sense::click()
        } else {
            Sense::hover()
        };
        let (rect, mut response) = ui.allocate_exact_size(desired, sense);

        // Selected label lookup
        let selected_item = self.items.iter().find(|it| &it.value == self.selected);
        let (display_text, is_placeholder) = if let Some(item) = selected_item {
            (item.label.clone(), false)
        } else if let Some(placeholder) = &self.placeholder {
            (placeholder.clone(), true)
        } else {
            (WidgetText::from(""), true)
        };

        // Trigger colors & visuals (shadcn/ui style)
        let border_stroke = if !self.enabled {
            Stroke::new(m.border_width, p.border)
        } else if is_open {
            Stroke::new(1.5, p.accent)
        } else if response.hovered() {
            Stroke::new(m.border_width, p.border_strong)
        } else {
            Stroke::new(m.border_width, p.border)
        };

        let bg = if !self.enabled {
            p.subtle
        } else if response.hovered() {
            crate::util::mix(p.field_bg, p.subtle, 0.5)
        } else {
            p.field_bg
        };

        let fg = if !self.enabled {
            p.text_disabled
        } else if is_placeholder {
            p.text_secondary
        } else {
            p.text_primary
        };

        let rounding = radius(m.radius_md);

        // Paint Trigger
        ui.painter().rect_filled(rect, rounding, bg);
        ui.painter()
            .rect_stroke(rect, rounding, border_stroke, StrokeKind::Inside);

        // Text galley with padding
        let text_font = theme.typography.font_id(crate::widgets::Size::Md);
        let galley = ui
            .scope(|ui| {
                ui.style_mut().visuals.override_text_color = Some(fg);
                display_text.into_galley(
                    ui,
                    Some(egui::TextWrapMode::Truncate),
                    rect.width() - 36.0,
                    text_font,
                )
            })
            .inner;

        let text_pos = pos2(
            rect.left() + 12.0,
            rect.center().y - galley.size().y / 2.0,
        );
        ui.painter().galley(text_pos, galley, fg);

        // Subtle chevron down on the right
        let chevron_center = pos2(rect.right() - 16.0, rect.center().y);
        let chevron_color = if self.enabled {
            p.text_secondary
        } else {
            p.text_disabled
        };
        paint_chevron_down(ui.painter(), chevron_center, chevron_color);

        // Popup Menu (shadcn/ui popover)
        let popup = Popup::menu(&response)
            .id(popup_id)
            .width(rect.width())
            .gap(4.0)
            .close_behavior(PopupCloseBehavior::CloseOnClick)
            .frame(egui::Frame {
                fill: p.surface,
                stroke: Stroke::new(m.border_width, p.border),
                corner_radius: radius(m.radius_md),
                inner_margin: Margin::same(4),
                shadow: Shadow {
                    color: if theme.dark_mode {
                        Color32::from_black_alpha(100)
                    } else {
                        Color32::from_black_alpha(24)
                    },
                    offset: [0, 4],
                    blur: 10,
                    spread: 0,
                },
                ..Default::default()
            });

        popup.show(|ui| {
            ui.set_min_width(rect.width() - 8.0);
            ScrollArea::vertical()
                .max_height(240.0)
                .auto_shrink([false, true])
                .show(ui, |ui| {
                    ui.set_min_width(ui.available_width());
                    ui.spacing_mut().item_spacing.y = 2.0;

                    for item in &self.items {
                        let is_active = &item.value == self.selected;
                        let item_h = 30.0;
                        let item_desired = vec2(ui.available_width(), item_h);

                        let (item_rect, item_response) =
                            ui.allocate_exact_size(item_desired, Sense::click());

                        let item_bg = if item_response.hovered() {
                            p.subtle
                        } else {
                            Color32::TRANSPARENT
                        };

                        let item_fg = if is_active {
                            p.text_primary
                        } else if item_response.hovered() {
                            p.text_primary
                        } else {
                            p.text_secondary
                        };

                        ui.painter().rect_filled(
                            item_rect,
                            radius(m.radius_sm),
                            item_bg,
                        );

                        // If active, paint checkmark on the left
                        if is_active {
                            let check_center =
                                pos2(item_rect.left() + 14.0, item_rect.center().y);
                            paint_check(ui.painter(), check_center, p.accent);
                        }

                        // Label text
                        let font_id = theme.typography.font_id(crate::widgets::Size::Md);
                        let galley = ui
                            .scope(|ui| {
                                ui.style_mut().visuals.override_text_color = Some(item_fg);
                                item.label.clone().into_galley(
                                    ui,
                                    Some(egui::TextWrapMode::Truncate),
                                    item_rect.width() - 32.0,
                                    font_id,
                                )
                            })
                            .inner;

                        let label_x = item_rect.left() + 28.0;
                        let label_pos = pos2(
                            label_x,
                            item_rect.center().y - galley.size().y / 2.0,
                        );
                        ui.painter().galley(label_pos, galley, item_fg);

                        if item_response.clicked() {
                            *self.selected = item.value.clone();
                            response.mark_changed();
                            Popup::close_all(ui.ctx());
                        }
                    }
                });
        });

        response
    }
}

/// Paint a clean, sharp vector chevron pointing down.
fn paint_chevron_down(painter: &egui::Painter, center: egui::Pos2, color: egui::Color32) {
    let stroke = egui::Stroke::new(1.4, color);
    let size = 3.2;
    painter.line_segment(
        [
            pos2(center.x - size, center.y - 1.2),
            pos2(center.x, center.y + 2.0),
        ],
        stroke,
    );
    painter.line_segment(
        [
            pos2(center.x, center.y + 2.0),
            pos2(center.x + size, center.y - 1.2),
        ],
        stroke,
    );
}

/// Paint a clean, sharp vector checkmark.
fn paint_check(painter: &egui::Painter, center: egui::Pos2, color: egui::Color32) {
    let stroke = egui::Stroke::new(1.5, color);
    painter.line_segment(
        [
            pos2(center.x - 3.5, center.y),
            pos2(center.x - 1.0, center.y + 2.5),
        ],
        stroke,
    );
    painter.line_segment(
        [
            pos2(center.x - 1.0, center.y + 2.5),
            pos2(center.x + 3.5, center.y - 3.0),
        ],
        stroke,
    );
}
