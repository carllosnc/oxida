//! Themed button with variants and sizes.

use egui::{pos2, vec2, Color32, Response, Sense, Stroke, StrokeKind, Ui, Widget, WidgetText};

use crate::theme;
use crate::util::{mix, radius};
use super::Size;

/// Visual variant of a [`Button`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonVariant {
    /// Accent background — the default, for the primary action.
    #[default]
    Primary,
    /// Surface background with a border.
    Secondary,
    /// Transparent background — low emphasis.
    Ghost,
    /// Danger background — destructive actions.
    Danger,
}

/// A themed button.
///
/// ```
/// # use oxida::prelude::*;
/// # fn demo(ui: &mut egui::Ui) {
/// ui.oxida().button("Save");
/// # }
/// ```
///
/// Or with full control:
///
/// ```
/// # use oxida::prelude::*;
/// # fn demo(ui: &mut egui::Ui) {
/// Button::new("Delete")
///     .variant(ButtonVariant::Danger)
///     .size(Size::Sm)
///     .ui(ui);
/// # }
/// ```
#[derive(Clone)]
pub struct Button {
    label: WidgetText,
    icon: Option<WidgetText>,
    icon_after: Option<WidgetText>,
    variant: ButtonVariant,
    size: Size,
    enabled: bool,
    fill_width: bool,
}

impl Button {
    /// Create a button with the given label.
    pub fn new(label: impl Into<WidgetText>) -> Self {
        Self {
            label: label.into(),
            icon: None,
            icon_after: None,
            variant: ButtonVariant::default(),
            size: Size::default(),
            enabled: true,
            fill_width: false,
        }
    }

    /// Primary button (accent background).
    pub fn primary(label: impl Into<WidgetText>) -> Self {
        Self::new(label).variant(ButtonVariant::Primary)
    }

    /// Secondary button (surface background with border).
    pub fn secondary(label: impl Into<WidgetText>) -> Self {
        Self::new(label).variant(ButtonVariant::Secondary)
    }

    /// Ghost button (transparent background until hovered).
    pub fn ghost(label: impl Into<WidgetText>) -> Self {
        Self::new(label).variant(ButtonVariant::Ghost)
    }

    /// Danger button (destructive actions).
    pub fn danger(label: impl Into<WidgetText>) -> Self {
        Self::new(label).variant(ButtonVariant::Danger)
    }

    /// Add a leading icon to the button.
    pub fn icon(mut self, icon: impl Into<WidgetText>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Add a leading icon to the button (alias for [`Self::icon`]).
    pub fn leading_icon(self, icon: impl Into<WidgetText>) -> Self {
        self.icon(icon)
    }

    /// Add a trailing icon to the button (after the label).
    pub fn icon_after(mut self, icon: impl Into<WidgetText>) -> Self {
        self.icon_after = Some(icon.into());
        self
    }

    /// Add a trailing icon to the button (alias for [`Self::icon_after`]).
    pub fn trailing_icon(self, icon: impl Into<WidgetText>) -> Self {
        self.icon_after(icon)
    }

    /// Set the visual variant. Defaults to [`ButtonVariant::Primary`].
    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set the control size. Defaults to [`Size::Md`].
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    /// Enable/disable the button. Disabled buttons don't respond to input.
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Stretch the button to fill the available width.
    pub fn fill_width(mut self, fill_width: bool) -> Self {
        self.fill_width = fill_width;
        self
    }
}

impl Widget for Button {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = theme::current(ui.ctx());
        let p = &theme.palette;
        let m = &theme.metrics;

        // Resolve colors first: the text color must be known before layout,
        // because galley colors are baked in at layout time.
        let (base, hover, active, fg, border) = match self.variant {
            ButtonVariant::Primary => (p.accent, p.accent_hover, p.accent_active, p.on_accent, p.accent),
            ButtonVariant::Secondary => (p.surface, p.subtle, p.border, p.text_primary, p.border),
            ButtonVariant::Ghost => (Color32::TRANSPARENT, p.subtle, p.border, p.text_primary, Color32::TRANSPARENT),
            ButtonVariant::Danger => (p.danger, p.danger_hover, mix(p.danger, Color32::BLACK, 0.2), p.on_danger, p.danger),
        };
        let label_color = if self.enabled {
            fg
        } else {
            p.text_disabled
        };

        let height = match self.size {
            Size::Sm => m.control_height_sm,
            Size::Md => m.control_height_md,
            Size::Lg => m.control_height_lg,
        };
        let pad_x = match self.size {
            Size::Sm => m.spacing_unit * 2.5,
            Size::Md => m.spacing_unit * 3.5,
            Size::Lg => m.spacing_unit * 4.5,
        };

        // Layout with `label_color` baked into the galley (the theme's
        // `override_text_color` would otherwise win over the paint color).
        // Labels use the theme's bold button family.
        let mut font_id = theme.typography.font_id(self.size);
        font_id.family = theme.typography.button_family.clone();

        let icon_font_size = match self.size {
            Size::Sm => 14.0,
            Size::Md => 16.0,
            Size::Lg => 18.0,
        };
        let icon_font_id = egui::FontId::new(icon_font_size, egui::FontFamily::Name(crate::fonts::PHOSPHOR.into()));

        let icon_galley = self.icon.map(|ic| {
            ui.scope(|ui| {
                ui.style_mut().visuals.override_text_color = Some(label_color);
                ic.into_galley(ui, Some(egui::TextWrapMode::Extend), f32::INFINITY, icon_font_id.clone())
            })
            .inner
        });

        let icon_after_galley = self.icon_after.map(|ic| {
            ui.scope(|ui| {
                ui.style_mut().visuals.override_text_color = Some(label_color);
                ic.into_galley(ui, Some(egui::TextWrapMode::Extend), f32::INFINITY, icon_font_id.clone())
            })
            .inner
        });

        let text_galley = ui
            .scope(|ui| {
                ui.style_mut().visuals.override_text_color = Some(label_color);
                self.label.into_galley(ui, Some(egui::TextWrapMode::Extend), f32::INFINITY, font_id)
            })
            .inner;

        let has_text = !text_galley.text().is_empty();
        let gap = (m.spacing_unit * 1.5).round();

        let mut content_width = 0.0;
        if let Some(ref ig) = icon_galley {
            content_width += ig.size().x;
            if has_text {
                content_width += gap;
            }
        }
        if has_text {
            content_width += text_galley.size().x;
        }
        if let Some(ref tag) = icon_after_galley {
            if has_text || icon_galley.is_some() {
                content_width += gap;
            }
            content_width += tag.size().x;
        }

        let mut desired = vec2(content_width + pad_x * 2.0, height);
        if self.fill_width {
            desired.x = ui.available_width().max(desired.x);
        }

        let sense = if self.enabled { Sense::click() } else { Sense::hover() };
        let (rect, response) = ui.allocate_exact_size(desired, sense);

        let (bg, border_color) = if self.enabled {
            let hover_t = ui
                .ctx()
                .animate_bool_with_time(response.id, response.hovered(), 0.10);
            let mut bg = mix(base, hover, hover_t);
            if response.is_pointer_button_down_on() {
                bg = active;
            }
            (bg, border)
        } else {
            let dis_bg = if self.variant == ButtonVariant::Ghost {
                Color32::TRANSPARENT
            } else {
                p.subtle
            };
            let dis_border = if self.variant == ButtonVariant::Ghost {
                Color32::TRANSPARENT
            } else {
                p.border
            };
            (dis_bg, dis_border)
        };

        // Paint ---------------------------------------------------------------
        // Modern, refined corner radius instead of a full-bleed pill.
        let corner_r = match self.size {
            Size::Sm => m.radius_sm,
            Size::Md => m.radius_md,
            Size::Lg => m.radius_md + 2.0,
        };
        let rounding = radius(corner_r);

        ui.painter().rect_filled(rect, rounding, bg);
        if border_color != Color32::TRANSPARENT {
            ui.painter().rect_stroke(
                rect,
                rounding,
                Stroke::new(m.border_width, border_color),
                StrokeKind::Inside,
            );
        }

        let mut curr_x = rect.center().x - content_width / 2.0;
        if let Some(ref ig) = icon_galley {
            let icon_pos = pos2(
                curr_x,
                rect.center().y - ig.size().y / 2.0,
            );
            ui.painter().galley(icon_pos, ig.clone(), label_color);
            curr_x += ig.size().x;
            if has_text {
                curr_x += gap;
            }
        }

        if has_text {
            let text_pos = pos2(
                curr_x,
                rect.center().y - text_galley.size().y / 2.0,
            );
            curr_x += text_galley.size().x;
            ui.painter().galley(text_pos, text_galley, label_color);
            if icon_after_galley.is_some() {
                curr_x += gap;
            }
        }

        if let Some(ref tag) = icon_after_galley {
            let icon_pos = pos2(
                curr_x,
                rect.center().y - tag.size().y / 2.0,
            );
            ui.painter().galley(icon_pos, tag.clone(), label_color);
        }

        response
    }
}
