//! Themed search input widget with leading icon, clear button, and shortcut badge.

use std::ops::Deref;
use egui::{
    pos2, vec2, Align2, FontFamily, FontId, Id, Response, Sense, Stroke,
    StrokeKind, TextEdit, TextStyle, Ui, UiBuilder, Widget, WidgetText,
};

use crate::icons;
use crate::theme;
use crate::util::{mix, radius};

/// Response returned by [`SearchInput::show`].
///
/// Implements [`Deref<Target = Response>`] so all standard egui response methods
/// ([`.clicked()`](Response::clicked), [`.changed()`](Response::changed),
/// [`.has_focus()`](Response::has_focus)) can be called directly.
#[derive(Clone, Debug)]
pub struct SearchInputResponse {
    /// The underlying egui [`Response`].
    pub response: Response,
    /// Whether the clear button was clicked this frame.
    pub cleared: bool,
    /// Whether Enter was pressed while the search input had focus.
    pub submitted: bool,
}

impl Deref for SearchInputResponse {
    type Target = Response;
    fn deref(&self) -> &Self::Target {
        &self.response
    }
}

/// A desktop search input widget styled after Linear and shadcn/ui.
///
/// Features:
/// - Leading Phosphor magnifying glass search icon.
/// - Themed focus ring and hover highlights.
/// - Clear button (`×`) when content is present.
/// - Optional keyboard shortcut badge (e.g. `Ctrl+K` or `⌘K`).
///
/// ```
/// # use oxida::prelude::*;
/// # fn demo(ui: &mut egui::Ui, mut query: String) {
/// ui.oxida()
///     .search_input(&mut query)
///     .placeholder("Pesquisar componentes...")
///     .shortcut("Ctrl+K")
///     .ui(ui);
/// # }
/// ```
pub struct SearchInput<'t> {
    text: &'t mut String,
    placeholder: Option<WidgetText>,
    shortcut: Option<WidgetText>,
    clearable: bool,
    width: Option<f32>,
    id_salt: Option<Id>,
}

impl<'t> SearchInput<'t> {
    /// Create a new search input bound to `text`.
    pub fn new(text: &'t mut String) -> Self {
        Self {
            text,
            placeholder: Some(WidgetText::from("Pesquisar...")),
            shortcut: None,
            clearable: true,
            width: None,
            id_salt: None,
        }
    }

    /// Set placeholder text shown when the input is empty.
    pub fn placeholder(mut self, placeholder: impl Into<WidgetText>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    /// Set a keyboard shortcut badge to show on the trailing edge (e.g. `"Ctrl+K"` or `"⌘K"`).
    pub fn shortcut(mut self, shortcut: impl Into<WidgetText>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    /// Whether to display a clear button (`×`) when the input is not empty. Defaults to `true`.
    pub fn clearable(mut self, clearable: bool) -> Self {
        self.clearable = clearable;
        self
    }

    /// Set explicit fixed width. Defaults to full available width.
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// Set an explicit ID salt.
    pub fn id_salt(mut self, salt: Id) -> Self {
        self.id_salt = Some(salt);
        self
    }

    /// Render the search input and return detailed [`SearchInputResponse`].
    pub fn show(self, ui: &mut Ui) -> SearchInputResponse {
        let theme = theme::current(ui.ctx());
        let p = &theme.palette;
        let m = &theme.metrics;

        let height = m.control_height_md;
        let width = self.width.unwrap_or_else(|| ui.available_width());
        let (rect, mut container_response) = ui.allocate_exact_size(vec2(width, height), Sense::click());

        let mut cleared = false;
        let mut submitted = false;

        if ui.is_rect_visible(rect) {
            // 1. Measure and position leading search icon (Phosphor icon font)
            let icon_pad_left = 10.0_f32;
            let icon_color = p.text_secondary;
            let icon_font = FontId::new(theme.typography.sm + 2.0, FontFamily::Name(crate::fonts::PHOSPHOR.into()));
            let icon_galley = ui.scope(|ui| {
                ui.style_mut().visuals.override_text_color = Some(icon_color);
                WidgetText::from(icons::MAGNIFYING_GLASS).into_galley(
                    ui,
                    Some(egui::TextWrapMode::Truncate),
                    f32::INFINITY,
                    icon_font,
                )
            }).inner;
            let icon_w = icon_galley.size().x;
            let icon_pos = pos2(
                rect.min.x + icon_pad_left,
                rect.center().y - icon_galley.size().y * 0.5,
            );

            // 2. Measure trailing elements (shortcut badge and clear button)
            let mut right_reserved = 8.0_f32;
            let show_clear = self.clearable && !self.text.is_empty();

            let clear_button_size = 18.0_f32;
            let clear_rect = if show_clear {
                let cr = egui::Rect::from_center_size(
                    pos2(rect.max.x - right_reserved - clear_button_size * 0.5, rect.center().y),
                    vec2(clear_button_size, clear_button_size),
                );
                right_reserved += clear_button_size + 6.0;
                Some(cr)
            } else {
                None
            };

            let shortcut_galley = self.shortcut.map(|sc| {
                let sc_font = FontId::new(theme.typography.xs, FontFamily::Name(crate::fonts::MEDIUM.into()));
                ui.scope(|ui| {
                    ui.style_mut().visuals.override_text_color = Some(p.text_secondary);
                    sc.into_galley(ui, Some(egui::TextWrapMode::Truncate), f32::INFINITY, sc_font)
                }).inner
            });

            let shortcut_rect = if let Some(ref sc_g) = shortcut_galley {
                let badge_w = sc_g.size().x + 8.0;
                let badge_h = 18.0_f32;
                let sr = egui::Rect::from_min_size(
                    pos2(rect.max.x - right_reserved - badge_w, rect.center().y - badge_h * 0.5),
                    vec2(badge_w, badge_h),
                );
                right_reserved += badge_w + 6.0;
                Some(sr)
            } else {
                None
            };

            // 3. Central TextEdit
            let text_left = rect.min.x + icon_pad_left + icon_w + 8.0;
            let text_right = (rect.max.x - right_reserved).max(text_left + 20.0);
            let text_rect = egui::Rect::from_min_max(
                pos2(text_left, rect.min.y),
                pos2(text_right, rect.max.y),
            );

            let edit_id = self.id_salt.unwrap_or_else(|| ui.scope_id().with("search_text"));
            let mut child_ui = ui.new_child(UiBuilder::new().max_rect(text_rect));

            let mut edit = TextEdit::singleline(self.text)
                .id(edit_id)
                .frame(egui::Frame::NONE)
                .min_size(vec2(text_rect.width(), height))
                .align(Align2::LEFT_CENTER)
                .font(TextStyle::Body)
                .desired_width(text_rect.width());

            if let Some(placeholder) = self.placeholder {
                edit = edit.hint_text(placeholder);
            }

            let edit_response = child_ui.add(edit);

            if edit_response.changed() {
                container_response.mark_changed();
            }

            let is_focused = edit_response.has_focus();
            if container_response.clicked() && !is_focused {
                edit_response.request_focus();
            }

            if is_focused && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                submitted = true;
            }

            // 4. Background and border
            let bg_color = p.field_bg;
            let border_color = if is_focused {
                p.accent
            } else if container_response.hovered() || edit_response.hovered() {
                mix(p.border, p.text_secondary, 0.4)
            } else {
                p.border
            };

            ui.painter().rect(
                rect,
                radius(m.radius_md),
                bg_color,
                Stroke::new(m.border_width, border_color),
                StrokeKind::Inside,
            );

            // Focus ring
            if is_focused {
                ui.painter().rect_stroke(
                    rect,
                    radius(m.radius_md),
                    Stroke::new(m.border_width * 1.5, p.accent),
                    StrokeKind::Inside,
                );
            }

            // Draw leading search icon
            ui.painter().galley(icon_pos, icon_galley, icon_color);

            // Draw trailing shortcut badge if present
            if let (Some(sr), Some(sc_g)) = (shortcut_rect, shortcut_galley) {
                ui.painter().rect(
                    sr,
                    radius(m.radius_sm),
                    p.subtle,
                    Stroke::new(m.border_width, mix(p.border, p.subtle, 0.4)),
                    StrokeKind::Inside,
                );
                let text_pos = pos2(
                    sr.center().x - sc_g.size().x * 0.5,
                    sr.center().y - sc_g.size().y * 0.5,
                );
                ui.painter().galley(text_pos, sc_g, p.text_secondary);
            }

            // Draw clear button if shown
            if let Some(cr) = clear_rect {
                let clear_resp = ui.interact(cr, edit_id.with("clear_btn"), Sense::click());
                let clear_hovered = clear_resp.hovered();

                if clear_hovered {
                    ui.painter().circle_filled(cr.center(), clear_button_size * 0.45, p.subtle);
                }

                let x_color = if clear_hovered {
                    p.text_primary
                } else {
                    p.text_secondary
                };

                let x_font = FontId::new(theme.typography.xs + 2.0, FontFamily::Name(crate::fonts::PHOSPHOR.into()));
                let x_galley = ui.scope(|ui| {
                    ui.style_mut().visuals.override_text_color = Some(x_color);
                    WidgetText::from(icons::X).into_galley(
                        ui,
                        Some(egui::TextWrapMode::Truncate),
                        f32::INFINITY,
                        x_font,
                    )
                }).inner;

                let x_pos = pos2(
                    cr.center().x - x_galley.size().x * 0.5,
                    cr.center().y - x_galley.size().y * 0.5,
                );
                ui.painter().galley(x_pos, x_galley, x_color);

                if clear_resp.clicked() {
                    self.text.clear();
                    cleared = true;
                    container_response.mark_changed();
                }
            }
        }

        SearchInputResponse {
            response: container_response,
            cleared,
            submitted,
        }
    }
}

impl Widget for SearchInput<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        self.show(ui).response
    }
}
