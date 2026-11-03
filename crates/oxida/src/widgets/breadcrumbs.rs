//! Themed breadcrumb navigation trail widget.

use egui::{
    pos2, vec2, Color32, FontFamily, FontId, Response, Sense, StrokeKind, Ui, Widget, WidgetText,
};

use crate::icons;
use crate::theme;
use crate::util::{mix, radius};

/// A single segment within a [`Breadcrumbs`] trail.
#[derive(Clone)]
pub struct BreadcrumbItem {
    pub(crate) label: WidgetText,
    pub(crate) icon: Option<WidgetText>,
}

impl BreadcrumbItem {
    /// Create a new breadcrumb item with text.
    pub fn new(label: impl Into<WidgetText>) -> Self {
        Self {
            label: label.into(),
            icon: None,
        }
    }

    /// Add an icon before the item label.
    pub fn icon(mut self, icon: impl Into<WidgetText>) -> Self {
        self.icon = Some(icon.into());
        self
    }
}

impl<T: Into<WidgetText>> From<T> for BreadcrumbItem {
    fn from(label: T) -> Self {
        Self::new(label)
    }
}

/// A breadcrumb navigation trail styled after modern desktop apps.
///
/// Indicates the current page's location within a navigational hierarchy.
///
/// ```
/// # use oxida::prelude::*;
/// # fn demo(ui: &mut egui::Ui) {
/// if let Some(clicked_idx) = Breadcrumbs::new()
///     .item_with_icon(icons::HOUSE, "Início")
///     .item("Projetos")
///     .active("Oxida")
///     .show(ui)
/// {
///     println!("Clicked breadcrumb segment: {clicked_idx}");
/// }
/// # }
/// ```
#[derive(Clone, Default)]
pub struct Breadcrumbs {
    items: Vec<BreadcrumbItem>,
    active_item: Option<BreadcrumbItem>,
    separator: Option<WidgetText>,
}

impl Breadcrumbs {
    /// Create an empty breadcrumbs trail.
    pub fn new() -> Self {
        Self::default()
    }

    /// Construct breadcrumbs from an iterator of labels.
    pub fn from_items<I, T>(items: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<BreadcrumbItem>,
    {
        Self {
            items: items.into_iter().map(Into::into).collect(),
            active_item: None,
            separator: None,
        }
    }

    /// Append an item to the navigation trail.
    pub fn item(mut self, label: impl Into<WidgetText>) -> Self {
        self.items.push(BreadcrumbItem::new(label));
        self
    }

    /// Append an item with an icon to the navigation trail.
    pub fn item_with_icon(mut self, icon: impl Into<WidgetText>, label: impl Into<WidgetText>) -> Self {
        self.items.push(BreadcrumbItem::new(label).icon(icon));
        self
    }

    /// Set the active (current/terminal) breadcrumb item.
    ///
    /// The active item is styled with primary emphasis and is non-clickable.
    pub fn active(mut self, label: impl Into<WidgetText>) -> Self {
        self.active_item = Some(BreadcrumbItem::new(label));
        self
    }

    /// Set custom separator widget (defaults to [`icons::CARET_RIGHT`]).
    pub fn separator(mut self, separator: impl Into<WidgetText>) -> Self {
        self.separator = Some(separator.into());
        self
    }

    /// Render breadcrumbs and return `Some(index)` if any clickable item was clicked.
    pub fn show(self, ui: &mut Ui) -> Option<usize> {
        let theme = theme::current(ui.ctx());
        let p = &theme.palette;
        let m = &theme.metrics;

        let total_items = self.items.len() + if self.active_item.is_some() { 1 } else { 0 };
        if total_items == 0 {
            return None;
        }

        let mut clicked_index = None;

        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 6.0;

            let font_regular = FontId::new(theme.typography.sm, FontFamily::Name(crate::fonts::MEDIUM.into()));
            let font_active = FontId::new(theme.typography.sm, FontFamily::Name(crate::fonts::SEMI_BOLD.into()));
            let sep_font = FontId::new(theme.typography.xs, FontFamily::Name(crate::fonts::REGULAR.into()));

            for (i, item) in self.items.iter().enumerate() {
                // Interactive link item
                let galley = ui.scope(|ui| {
                    ui.style_mut().visuals.override_text_color = Some(p.text_secondary);
                    item.label.clone().into_galley(ui, Some(egui::TextWrapMode::Truncate), f32::INFINITY, font_regular.clone())
                }).inner;

                let icon_galley = item.icon.as_ref().map(|ic| {
                    ui.scope(|ui| {
                        ui.style_mut().visuals.override_text_color = Some(p.text_secondary);
                        ic.clone().into_galley(ui, Some(egui::TextWrapMode::Truncate), f32::INFINITY, font_regular.clone())
                    }).inner
                });

                let icon_w = icon_galley.as_ref().map_or(0.0, |g| g.size().x + 4.0);
                let item_w = galley.size().x + icon_w + 8.0;
                let item_h = 24.0_f32;

                let (rect, resp) = ui.allocate_exact_size(vec2(item_w, item_h), Sense::click());

                if ui.is_rect_visible(rect) {
                    let painter = ui.painter();
                    let is_hovered = resp.hovered();

                    if is_hovered {
                        painter.rect(
                            rect,
                            radius(m.radius_sm),
                            p.subtle,
                            egui::Stroke::NONE,
                            StrokeKind::Inside,
                        );
                    }

                    let text_color = if is_hovered {
                        p.text_primary
                    } else {
                        p.text_secondary
                    };

                    let mut curr_x = rect.min.x + 4.0;
                    if let Some(ig) = icon_galley {
                        let ipos = pos2(curr_x, rect.center().y - ig.size().y * 0.5);
                        painter.galley(ipos, ig, text_color);
                        curr_x += icon_w;
                    }

                    let tpos = pos2(curr_x, rect.center().y - galley.size().y * 0.5);
                    painter.galley(tpos, galley, text_color);
                }

                if resp.clicked() {
                    clicked_index = Some(i);
                }

                // Separator
                render_separator(ui, &self.separator, p.text_secondary, sep_font.clone());
            }

            // Render terminal active item if present
            if let Some(ref active) = self.active_item {
                let galley = ui.scope(|ui| {
                    ui.style_mut().visuals.override_text_color = Some(p.text_primary);
                    active.label.clone().into_galley(ui, Some(egui::TextWrapMode::Truncate), f32::INFINITY, font_active)
                }).inner;

                let icon_galley = active.icon.as_ref().map(|ic| {
                    ui.scope(|ui| {
                        ui.style_mut().visuals.override_text_color = Some(p.text_primary);
                        ic.clone().into_galley(ui, Some(egui::TextWrapMode::Truncate), f32::INFINITY, font_regular.clone())
                    }).inner
                });

                let icon_w = icon_galley.as_ref().map_or(0.0, |g| g.size().x + 4.0);
                let item_w = galley.size().x + icon_w + 8.0;
                let item_h = 24.0_f32;

                let (rect, _) = ui.allocate_exact_size(vec2(item_w, item_h), Sense::hover());

                if ui.is_rect_visible(rect) {
                    let painter = ui.painter();
                    let mut curr_x = rect.min.x + 4.0;
                    if let Some(ig) = icon_galley {
                        let ipos = pos2(curr_x, rect.center().y - ig.size().y * 0.5);
                        painter.galley(ipos, ig, p.text_primary);
                        curr_x += icon_w;
                    }
                    let tpos = pos2(curr_x, rect.center().y - galley.size().y * 0.5);
                    painter.galley(tpos, galley, p.text_primary);
                }
            }
        });

        clicked_index
    }
}

fn render_separator(ui: &mut Ui, custom_sep: &Option<WidgetText>, color: Color32, font: FontId) {
    let sep_text = custom_sep.clone().unwrap_or_else(|| WidgetText::from(icons::CARET_RIGHT));
    let sep_color = mix(color, Color32::TRANSPARENT, 0.4);

    let galley = ui.scope(|ui| {
        ui.style_mut().visuals.override_text_color = Some(sep_color);
        sep_text.into_galley(ui, Some(egui::TextWrapMode::Truncate), f32::INFINITY, font)
    }).inner;

    let (rect, _) = ui.allocate_exact_size(vec2(galley.size().x + 2.0, 24.0), Sense::hover());
    if ui.is_rect_visible(rect) {
        let pos = pos2(rect.center().x - galley.size().x * 0.5, rect.center().y - galley.size().y * 0.5);
        ui.painter().galley(pos, galley, sep_color);
    }
}

impl Widget for Breadcrumbs {
    fn ui(self, ui: &mut Ui) -> Response {
        let (_rect, resp) = ui.allocate_exact_size(vec2(0.0, 24.0), Sense::hover());
        self.show(ui);
        resp
    }
}
